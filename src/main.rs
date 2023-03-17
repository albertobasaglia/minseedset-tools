mod bigmmodel;
mod newmodel;
mod pathway;
mod timesetmodel;

use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

use bigmmodel::build_bigm_model;
use newmodel::build_newmodel_model;
use timesetmodel::build_timeset_model;

use clap::Parser;
use clap::ValueEnum;
use log::info;
use log::trace;
use lp_modeler::format::lp_format::LpFileFormat;

use crate::pathway::{Compound, Pathway, Reaction};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum ModelType {
    Timeset,
    Bigm,
    New,
}

#[derive(Parser)]
struct Args {
    mode: ModelType,
    filename: PathBuf,
    #[arg(long, short, default_value_t = 10)]
    time: u32,
    model_name: PathBuf,
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

fn main() {
    env_logger::init();
    let args = Args::parse();

    let file = File::open(args.filename).expect("Can't open file");

    let buffer_reader = BufReader::new(file);

    let pathway = parse_file(buffer_reader);

    let problem = match args.mode {
        ModelType::Bigm => build_bigm_model(pathway, args.time as i32),
        ModelType::Timeset => build_timeset_model(pathway, args.time as usize + 2),
        ModelType::New => build_newmodel_model(pathway, args.time as i32),
    };
    // let cc = pathway.get_compounds_count() + pathway.get_reactions_count();

    info!("Exporting model");

    let binding = args.model_name.into_os_string().into_string().unwrap();
    let model_path = binding.as_str();

    problem.write_lp(model_path).expect("Can't write model");
}
