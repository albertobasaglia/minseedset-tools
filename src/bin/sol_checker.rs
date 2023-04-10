use clap::Parser;
use fast_model::pw::pathway::Pathway;
use log::info;
use serde::{Deserialize, Serialize};
use serde_xml_rs::from_reader;
use std::collections::HashSet;
use std::path::PathBuf;
use std::{fs::File, io::BufReader};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename = "CPLEXSolution")]
struct Solution {
    #[serde(rename = "header")]
    header: Header,
    #[serde(rename = "linearConstraints")]
    constraints: LinearConstraints,
    #[serde(rename = "variables")]
    variables: Variables,
    #[serde(rename = "objectiveValues")]
    objective: Objective,
}

#[derive(Debug, Deserialize, Serialize)]
struct Header {
    #[serde(rename = "problemName")]
    problem_name: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct LinearConstraints {
    #[serde(rename = "constraint")]
    constraints: Vec<Constraint>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Constraint {
    #[serde(rename = "name")]
    name: String,
    #[serde(rename = "index")]
    index: usize,
    #[serde(rename = "slack")]
    slack: f32,
}

#[derive(Debug, Deserialize, Serialize)]
struct Variables {
    #[serde(rename = "variable")]
    variables: Vec<Variable>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Variable {
    #[serde(rename = "name")]
    name: String,
    #[serde(rename = "index")]
    index: usize,
    #[serde(rename = "value")]
    value: f32,
}

#[derive(Debug, Deserialize, Serialize)]
struct Objective {
    #[serde(rename = "objective")]
    objective: Vec<ObjectiveValue>,
}

#[derive(Debug, Deserialize, Serialize)]
struct ObjectiveValue {
    #[serde(rename = "name")]
    name: String,
    #[serde(rename = "index")]
    index: usize,
    #[serde(rename = "value")]
    value: f32,
}

#[derive(Parser)]
struct Args {
    /// Json model file
    model: PathBuf,

    /// CPLEX solution file
    solution: PathBuf,
}

fn main() {
    env_logger::init();
    let args = Args::parse();

    let model_name = args.model;
    let solution_name = args.solution;

    let model_file = File::open(model_name).expect("Can't open model file");
    let model_reader = BufReader::new(model_file);

    let solution_file = File::open(solution_name).expect("Can't open solution file");
    let read_sol = BufReader::new(solution_file);
    let sol: Solution = from_reader(read_sol).expect("Can't read xml");

    let xvals = sol
        .variables
        .variables
        .iter()
        .filter(|x| x.name.starts_with("x") && x.value == 1.0);

    // Add elements in a set and do "apply the reactions until
    // they have no effect anymore or the set is complete".

    let pw: Pathway = serde_json::from_reader(model_reader).expect("Error parsing json");
    info!("Pathway contains {} reactions", pw.get_reactions_count());
    info!("Pathway contains {} compounds", pw.get_compounds_count());

    let mut in_set = HashSet::<u32>::new();
    for xval in xvals {
        in_set.insert(
            xval.name
                .strip_prefix("x")
                .expect("Invalid index")
                .parse()
                .expect("Invalid index number"),
        );
    }

    let mut iteration = 0u32;
    let mut to_add = Vec::<u32>::new();
    loop {
        for reac in pw.get_reactions() {
            let mut all_substrate = true;
            for s in reac.get_substrate() {
                if !in_set.contains(s) {
                    all_substrate = false;
                    break;
                }
            }
            if all_substrate {
                for p in reac.get_product() {
                    if !in_set.contains(p) {
                        to_add.push(p.clone());
                    }
                }
            }
        }

        iteration += 1;

        if to_add.len() == 0 || in_set.len() == pw.get_compounds_count() {
            break;
        }

        in_set.extend(to_add.iter());
        to_add.clear();
    }
    info!("Completed {} iterations, ", iteration);
    if in_set.len() == pw.get_compounds_count() {
        info!("set is reachable.");
    } else {
        info!("set is unreachable, completed full iteration with no effect.");
    }
    print!("{}", iteration);
}
