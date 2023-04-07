use std::cmp::min;
use std::fs::File;
use std::io::BufWriter;
use std::path::PathBuf;

use clap::Parser;
use clap::ValueEnum;
use fast_model::models::bigmmodel::build_bigm_model;
use fast_model::models::newmodel::build_newmodel_model;
use fast_model::models::timesetmodel::build_timeset_model;
use fast_model::parsers::parsepddl::parse_pddl;
use fast_model::parsers::parsereadable::parse_readable;
use fast_model::pw::pathway::Pathway;
use log::info;
use log::trace;
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
    time: i32,

    /// Name of the output file
    model_name: PathBuf,

    /// Input file type
    input_type: InputType,

    /// Split multiple-product reactions
    #[arg(short, long)]
    split: bool,

    /// Join duplicate and dominated reactions
    #[arg(short, long)]
    join_duplicates: bool,

    /// Export the generated model for further operations
    #[arg(short, long)]
    export: bool,
}

fn print_count(pathway: &Pathway) {
    let cc = pathway.get_compounds_count();
    let rc = pathway.get_reactions_count();

    info!("Compound count: {}, Reaction count: {}", cc, rc);
}

fn main() {
    env_logger::init();
    let args = Args::parse();

    let mut pathway = match args.input_type {
        InputType::Readable => parse_readable(args.filename),
        InputType::PDDL => parse_pddl(args.filename),
    };

    print_count(&pathway);

    if args.split {
        info!("Splitting multiple product reactions");
        let count = pathway.split_multiple_product();
        info!("Split {} reactions", count);
        print_count(&pathway);
    }

    if args.join_duplicates {
        info!("Removing duplicate reactions");
        let count = pathway.join_duplicates();
        info!("Removed {} reactions", count);
        print_count(&pathway);

        info!("Removing dominated reactions");
        let count = pathway.join_dominated();
        info!("Removed {} reactions", count);
        print_count(&pathway);
    }

    trace!("{:?}", pathway);

    let mut time_m = args.time;

    if args.time == -1 {
        info!("Using min{{#read, #comp}} as time instants");
        time_m = min(pathway.get_reactions_count(), pathway.get_compounds_count()) as i32;
    }

    let problem = match args.mode {
        ModelType::Bigm => build_bigm_model(&pathway, time_m),
        ModelType::Timeset => build_timeset_model(&pathway, time_m as usize + 2),
        ModelType::New => build_newmodel_model(&pathway, time_m),
    };

    info!("Exporting model");

    let binding = args.model_name.into_os_string().into_string().unwrap();
    let model_path = binding.as_str();

    problem.write_lp(model_path).expect("Can't write model");
    if args.export {
        let model_out = File::create("model.json").expect("Can't open file");
        let writer = BufWriter::new(model_out);
        serde_json::to_writer_pretty(writer, &pathway);
    }
}
