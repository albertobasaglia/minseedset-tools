use fast_model::pw::pathway::Pathway;
use log::info;
use log::trace;
use serde::Deserializer;
use serde::Serializer;
use serde::{Deserialize, Serialize};
use serde_xml_rs::from_reader;
use std::collections::HashSet;
use std::io::BufRead;
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

fn main() {
    env_logger::init();

    let model_name = "model.json";
    let model_file = File::open(model_name).expect("Can't open model file");
    let model_reader = BufReader::new(model_file);

    let solution_name = "sol.txt";
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
    loop {
        let mut has_effect = false;
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
                    if in_set.insert(p.clone()) {
                        has_effect = true;
                    }
                }
            }
        }
        if !has_effect || in_set.len() == pw.get_compounds_count() {
            break;
        }
        iteration += 1;
    }
    print!("Completed {} iterations, ", iteration);
    if in_set.len() == pw.get_compounds_count() {
        println!("set is reachable.");
    } else {
        println!("set is unreachable, completed full iteration with no effect.");
    }
}
