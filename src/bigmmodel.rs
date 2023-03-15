use crate::pathway::Pathway;
use log::info;
use lp_modeler::dsl::{LpBinary, LpExpression, LpInteger, LpOperations, LpProblem};

pub fn build_bigm_model(pathway: Pathway, m: i32) -> LpProblem {
    info!("Building Big-M model with M = {}", m);
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

        if reaction.get_product().len() > 1 {
            panic!("This model doesn't allow multiple reactions products");
        }

        for sub in reaction.get_substrate() {
            let sub_usize: usize = sub.to_owned() as usize;
            reac_requires_comp[reaction.get_id() as usize].push(sub_usize as u32);
        }
    }

    // i index
    let mut vars_x = Vec::<LpBinary>::new();
    let mut vars_tm = Vec::<LpInteger>::new();

    // j index
    let mut vars_u = Vec::<LpBinary>::new();
    let mut vars_tr = Vec::<LpInteger>::new();

    info!("Generating variables");
    let mut problem = LpProblem::new("MSS", lp_modeler::dsl::LpObjective::Minimize);

    for i in 0..cs {
        vars_x.push(LpBinary::new(format!("x{}", i).as_str()));
        vars_tm.push(LpInteger::new(format!("tm{}", i).as_str()));
    }

    for j in 0..rs {
        let u = LpBinary::new(format!("u{}", j).as_str());

        vars_u.push(u);
        vars_tr.push(LpInteger::new(format!("tr{}", j).as_str()));
    }

    info!("Generating constraints");

    // target function
    for var_x in &vars_x {
        problem += var_x;
    }

    info!("0/4");

    // x_i + sum (pij uj) >= 1 for all i
    for i in 0..cs {
        let xi = &vars_x[i];

        let mut sum_vars = Vec::<&LpBinary>::new();

        for a in &comp_produced_by_reac[i] {
            let reac = &vars_u[a.to_owned() as usize];
            sum_vars.push(reac);
        }

        let mut expr: LpExpression = xi.try_into().unwrap();

        for sv in sum_vars {
            expr = expr + sv;
        }

        problem += expr.ge(1);
    }

    info!("1/4");

    // tc rij = 1

    for (reaction, compounds) in reac_requires_comp.iter().enumerate() {
        for compound in compounds {
            let tmi = &vars_tm[compound.to_owned() as usize];
            let trj = &vars_tr[reaction];
            let uj = &vars_u[reaction];
            let xi = &vars_x[compound.to_owned() as usize];

            problem += (tmi + 1).le(trj + m - m * uj + m * xi);
        }
    }

    info!("2/4");

    // tc pij = 1

    for (compound, reactions) in comp_produced_by_reac.iter().enumerate() {
        for reaction in reactions {
            let trj = &vars_tr[reaction.to_owned() as usize];
            let tmi = &vars_tm[compound];
            let uj = &vars_u[reaction.to_owned() as usize];

            problem += (trj).le(tmi + m - m * uj);
        }
    }

    info!("3/4");

    for i in 0..cs {
        let tm = &vars_tm[i];
        problem += tm.le(m);
    }

    for j in 0..rs {
        let tr = &vars_tr[j];
        problem += tr.le(m);
    }

    info!("4/4");

    problem
}
