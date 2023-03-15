use crate::pathway::Pathway;
use log::info;
use log::trace;
use lp_modeler::dsl::{LpBinary, LpExpression, LpInteger, LpOperations, LpProblem};

pub fn build_newmodel_model(pathway: Pathway, m: i32) -> LpProblem {
    info!("Building NEW model with M = {}", m);
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

    // i index
    let mut vars_x = Vec::<LpBinary>::new();
    let mut vars_t = Vec::<LpInteger>::new();

    // ij index
    let mut vars_u = Vec::<Vec<LpBinary>>::new();

    info!("Generating variables");
    let mut problem = LpProblem::new("MSS", lp_modeler::dsl::LpObjective::Minimize);

    for i in 0..cs {
        vars_x.push(LpBinary::new(format!("x{}", i).as_str()));
        vars_t.push(LpInteger::new(format!("t{}", i).as_str()));
        vars_u.push(vec![]);
    }

    for (compound, cr) in comp_produced_by_reac.iter().enumerate() {
        for reac in cr {
            trace!("Compound {} produced by reaction {}", compound, reac);
            let u_bj = LpBinary::new(format!("u{}_{}", compound, reac).as_str());

            // Generate the constraint

            for req in &reac_requires_comp[reac.to_owned() as usize] {
                trace!("\tthat requires compound {}", req);
                let t_a = &vars_t[req.to_owned() as usize];
                let t_b = &vars_t[compound];
                let x_a = &vars_x[req.to_owned() as usize];
                // t_a + 1 <= t_b + M (x_a) + M (1 - u_bj)
                problem += t_a.le(-1 + t_b + m * x_a + m * (1 - &u_bj));
            }
            vars_u[compound].push(u_bj);
        }
    }

    info!("Generating constraints");

    // target function
    for var_x in &vars_x {
        problem += var_x;
    }

    // ti <= M
    for ti in &vars_t {
        problem += ti.le(m);
    }

    for i in 0..cs {
        let xi = &vars_x[i];
        let mut left_side: LpExpression = xi.try_into().unwrap();
        for a in &vars_u[i] {
            left_side += a;
        }
        problem += left_side.ge(1);
    }

    problem
}
