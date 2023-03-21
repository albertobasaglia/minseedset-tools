mod bigmmodel;
mod newmodel;
mod parsepddl;
mod parsereadable;
mod pathway;
mod timesetmodel;

use std::path::PathBuf;

use bigmmodel::build_bigm_model;
use newmodel::build_newmodel_model;
use parsepddl::parse_pddl;
use parsereadable::parse_readable;
use timesetmodel::build_timeset_model;

use clap::Parser;
use clap::ValueEnum;
use log::info;
use lp_modeler::format::lp_format::LpFileFormat;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum ModelType {
    Timeset,
    Bigm,
    New,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum InputType {
    Readable,
    PDDL,
}

#[derive(Parser)]
struct Args {
    /// Generated model type
    mode: ModelType,

    /// Name of the file to read
    filename: PathBuf,

    /// T/M of the model
    #[arg(long, short, default_value_t = 10)]
    time: u32,

    /// Name of the output file
    model_name: PathBuf,

    /// Input file type
    input_type: InputType,

    /// Split multiple-product reactions
    #[arg(short, long)]
    split: bool,

    /// Join duplicate reactions
    #[arg(short, long)]
    join_duplicates: bool,
}

fn main() {
    env_logger::init();
    let args = Args::parse();

    let pathway = match args.input_type {
        InputType::Readable => parse_readable(args.filename),
        InputType::PDDL => parse_pddl(args.filename),
    };

    // TODO implement split and join_duplicates

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
