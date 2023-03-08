mod pathway;

use std::{ fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

use clap::Parser;
use log::info;
use log::trace;
use lp_modeler::{
    dsl::{LpBinary, LpExpression, LpOperations, LpProblem},
    format::lp_format::LpFileFormat,
};

use crate::pathway::{Compound, Pathway, Reaction};

#[derive(Parser)]
struct Args {
    filename: PathBuf,
}

fn read_u32_from_optres(val: Option<&String>) -> u32 {
    val.expect("Parse error")
        .as_str()
        .parse()
        .expect("Parse error")
}

fn parse_file(reader: BufReader<File>) -> Pathway {
    let mut pathway = Pathway::new();

    let lines: Vec<String> = reader
        .lines()
        .collect::<Result<Vec<String>, std::io::Error>>()
        .expect("Iterator error");

    let mut it = lines.iter();

    let compounds_count: usize = read_u32_from_optres(it.next()).try_into().unwrap();

    info!("File contains {} compounds", compounds_count);

    let comps_it = it.clone().take(compounds_count);

    let mut compound_id = 0u32;
    for comp in comps_it {
        let c = Compound::new(compound_id, comp.clone());

        trace!("Added: {:?}", c);
        pathway.add_compound(c);

        compound_id += 1;
    }

    let mut reacs_it = it.clone().skip(compounds_count);
    let reactions_count: usize = read_u32_from_optres(reacs_it.next()).try_into().unwrap();
    info!("File contains {} reactions", reactions_count);

    let mut reaction_id = 0;

    while let Some(reac) = reacs_it.next() {
        let name = reac;

        let mut reaction = Reaction::new(reaction_id, name.clone());

        let substrate_size: u32 = reacs_it.next().unwrap().parse().expect("Format error!");
        for _ in 0..substrate_size {
            let compound = reacs_it.next().unwrap();
            let compound_id = pathway.get_compound_id(compound);
            reaction.add_substrate(compound_id);
        }

        let product_size: u32 = reacs_it.next().unwrap().parse().expect("Format error!");
        for _ in 0..product_size {
            let compound = reacs_it.next().unwrap();
            let compound_id = pathway.get_compound_id(compound);
            reaction.add_product(compound_id);
        }

        reaction_id += 1;

        trace!("Added: {:?}", reaction);
        pathway.add_reaction(reaction);
    }
    return pathway;
}

const T: usize = 30;

fn build_model(pathway: Pathway) -> LpProblem {
    info!("Building model");
    let rs = pathway.get_reactions_count();
    let cs = pathway.get_compounds_count();

    let mut comp_produced_by_reac = Vec::<Vec<u32>>::with_capacity(cs);
    let mut reac_requires_comp = Vec::<Vec<u32>>::with_capacity(rs);

    for _ in 0..cs {
        comp_produced_by_reac.push(vec![]);
    }

    for _ in 0..rs {
        reac_requires_comp.push(vec![]);
    }

    for reaction in pathway.get_reactions() {
        for prod in reaction.get_product() {
            let prod_usize: usize = prod.to_owned() as usize;
            comp_produced_by_reac[prod_usize].push(reaction.get_id());
        }

        for sub in reaction.get_substrate() {
            let sub_usize: usize = sub.to_owned() as usize;
            reac_requires_comp[reaction.get_id() as usize].push(sub_usize as u32);
        }
    }

    let mut vars_x = Vec::<LpBinary>::new();
    let mut vars_d = Vec::<[LpBinary; T]>::new();
    let mut vars_s = Vec::<[LpBinary; T]>::new();

    info!("Generating variables");
    // Create vars_x and vars_d
    for i in 0..cs {
        vars_x.push(LpBinary::new(format!("x{}", i).as_str()));

        let entry: [LpBinary; T] = (0..T)
            .into_iter()
            .map(|x| LpBinary::new(format!("d{}_{}", i, x).as_str()))
            .collect::<Vec<LpBinary>>()
            .try_into()
            .unwrap();

        vars_d.push(entry);
    }

    // Create vars_s
    for j in 0..rs {
        let entry: [LpBinary; T] = (0..T)
            .into_iter()
            .map(|x| LpBinary::new(format!("s{}_{}", j, x).as_str()))
            .collect::<Vec<LpBinary>>()
            .try_into()
            .unwrap();

        vars_s.push(entry);
    }

    let mut problem = LpProblem::new("MSS", lp_modeler::dsl::LpObjective::Minimize);

    info!("Generating constraints");

    // target function
    for var_x in &vars_x {
        problem += var_x;
    }

    info!("0/4");

    // d_i0 = x_i
    for i in 0..cs {
        let left = &vars_d[i][0];
        let right = &vars_x[i];
        problem += left.equal(right);
    }

    info!("1/4");

    // d_i(T-1) = 1
    for i in 0..cs {
        let left = &vars_d[i][T - 1];
        problem += left.equal(1);
    }

    info!("2/4");

    // d_it >= s_jt
    for (reaction, compounds) in reac_requires_comp.iter().enumerate() {
        for compound in compounds {
            for t in 0..T {
                let left = &vars_d[compound.to_owned() as usize][t];
                let right = &vars_s[reaction][t];
                problem += left.ge(right);
            }
        }
    }

    info!("3/4");

    for i in 0..cs {
        for t in 1..T {
            let left = &vars_d[i][t];
            let right = &vars_d[i][t - 1];

            let mut right_vars = Vec::<&LpBinary>::new();

            for reaction in &comp_produced_by_reac[i] {
                let other_right = &vars_s[reaction.to_owned() as usize][t - 1];
                right_vars.push(other_right);
            }

            let mut right_expr: LpExpression = right.try_into().unwrap();
            for rv in right_vars {
                right_expr = right_expr + rv;
            }

            problem += left.le(right_expr);
        }
    }

    info!("4/4");

    problem
}

fn main() {
    env_logger::init();
    let args = Args::parse();

    let file = File::open(args.filename).expect("Can't open file");

    let buffer_reader = BufReader::new(file);

    let pathway = parse_file(buffer_reader);

    let problem = build_model(pathway);

    info!("Exporting model");

    problem.write_lp("out.lp").expect("Can't write model");
}
