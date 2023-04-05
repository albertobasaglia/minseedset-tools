use serde::Deserializer;
use serde::Serializer;
use serde::{Deserialize, Serialize};
use serde_xml_rs::from_reader;
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
    let file_name = "sol.txt";
    let file = File::open(file_name).expect("Can't open file");
    let read = BufReader::new(file);
    let sol: Solution = from_reader(read).expect("Can't read xml");

    let xval = sol
        .variables
        .variables
        .iter()
        .filter(|x| x.name.starts_with("x") && x.value == 1.0);

    let seed_count = xval.count();
    println!("{} elements in the seed set", seed_count);
    // Add elements in a set and do the same "apply the reactions until
    // they have no effect anymore or the set is complete".
}
