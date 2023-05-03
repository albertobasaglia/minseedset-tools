use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

use log::info;
use log::trace;

use crate::pw::Compound;
use crate::pw::Pathway;
use crate::pw::Reaction;

fn read_u32_from_optres(val: Option<&String>) -> u32 {
    val.expect("Parse error")
        .as_str()
        .parse()
        .expect("Parse error")
}

/// Legge un pathway da un file di tipo .read
pub fn parse_readable(input: PathBuf) -> Pathway {
    let file = File::open(input).expect("Can't open file");

    let buffer_reader = BufReader::new(file);

    return parse_readable_internal(buffer_reader);
}

fn parse_readable_internal(reader: BufReader<File>) -> Pathway {
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
