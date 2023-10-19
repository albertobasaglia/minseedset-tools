use std::cmp::min;
use std::fs::File;
use std::io::BufWriter;
use std::path::PathBuf;

use clap::Parser;
use clap::ValueEnum;
use log::info;
use log::trace;
use lp_modeler::format::lp_format::LpFileFormat;
use msstools::models::bigmmodel::build_bigm_model;
use msstools::models::newmodel::build_newmodel_model;
use msstools::models::timesetmodel::build_timeset_model;
use msstools::parsers::parsepddl::parse_pddl;
use msstools::parsers::parsereadable::parse_readable;
use msstools::pw::Pathway;
use serde_json::to_writer_pretty;

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

    /// String representing the preprocessing to execute
    ///
    /// s: split,
    /// d: remove duplicates,
    /// D: remove dominated
    preprocessing_string: Option<String>,

    /// Export the generated model for further operations
    #[arg(long)]
    json_model: Option<PathBuf>,
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

    if let Some(json_path) = &args.json_model {
        let ext_path = json_path.with_extension("pre.json");
        info!("Writing pre-pp json model to {}", ext_path.display());
        let model_out = File::create(ext_path).expect("Can't open file");
        let writer = BufWriter::new(model_out);
        to_writer_pretty(writer, &pathway).expect("Model writing failed");
    }

    if let Some(pps) = args.preprocessing_string {
        let mut total_count = 1;
        let mut cycle_count = 1;
        while total_count > 0 {
            total_count = 0;
            info!("==== Preprocessing cycle #{} ====", cycle_count);
            cycle_count += 1;
            for pre in pps.chars() {
                match pre {
                    'd' => {
                        let count = pathway.join_duplicates();
                        info!("Joining duplicates removed {} reactions", count);
                        total_count += count;
                        // print_count(&pathway);
                    }
                    'P' => {
                        let count = pathway.join_dominated_product();
                        info!("Joining p-dominated removed {} reactions", count);
                        total_count += count;
                        // print_count(&pathway);
                    }
                    'S' => {
                        let count = pathway.join_dominated_substrate();
                        info!("Joining s-dominated removed {} reactions", count);
                        total_count += count;
                        // print_count(&pathway);
                    }
                    'm' => {
                        let count = pathway.merge_reactions();
                        info!("Merged {} reactions", count);
                        total_count += count;
                        // print_count(&pathway);
                    }
                    _ => {
                        info!("unknown preprocessing {}", pre);
                    }
                }
            }
        }
    }

    if let Some(json_path2) = args.json_model {
        let ext_path = json_path2.with_extension("post.json");
        info!("Writing post-pp json model to {}", ext_path.display());
        let model_out = File::create(ext_path).expect("Can't open file");
        let writer = BufWriter::new(model_out);
        to_writer_pretty(writer, &pathway).expect("Model writing failed");
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
}
