use std::{fs::File, io::Read, path::PathBuf};

use log::trace;

use crate::pw::Compound;
use crate::pw::Pathway;
use crate::pw::Reaction;

/// Legge un pathway da un file .pddl
pub fn parse_pddl(input: PathBuf) -> Pathway {
    let mut pathway = Pathway::new();

    let mut file = File::open(input).expect("Can't open file");

    let mut buffer = String::new();

    file.read_to_string(&mut buffer).expect("Can't read file");

    let binding = buffer.replace("\r", "").replace("\t", "").replace(" ", "");
    let mut entries = binding.split("\n");

    let mut reaction_counter = 0;
    let mut compound_counter = 0;

    let mut current_substrate: Vec<u32> = vec![];
    let mut current_product: Vec<u32> = vec![];
    let mut current_name = String::new();

    let mut has_reaction = false;

    let mut reading_substrate = true;
    let mut reading_reaction = false;

    while let Some(line) = entries.next() {
        trace!("Read: {}", line);
        if line.starts_with("(:actionreaction_") {
            trace!("Found new reaction");
            if has_reaction {
                trace!("Saving old reaction: {}", current_name);
                let mut reac = Reaction::new(reaction_counter, current_name.clone());
                trace!("\tSubstrate:");
                for sub in &current_substrate {
                    trace!("\t\t{}", sub);
                    reac.add_substrate(sub.clone());
                }
                trace!("\tProduct:");
                for prod in &current_product {
                    trace!("\t\t{}", prod);
                    reac.add_product(prod.clone());
                }

                pathway.add_reaction(reac);

                reaction_counter += 1;
            }

            current_substrate.clear();
            current_product.clear();

            current_name = line.strip_prefix("(:actionreaction_").unwrap().to_string();
            reading_reaction = true;
            has_reaction = true;
        } else if line.starts_with("(C") {
            if reading_reaction {
                let id = match pathway.get_compound_option(&line.to_string()) {
                    Some(id) => id,
                    None => {
                        let cur = compound_counter;
                        let cmp = Compound::new(cur, line.to_string());
                        pathway.add_compound(cmp);
                        compound_counter += 1;
                        cur
                    }
                };
                trace!("Adding compound with id: {}", id);
                if reading_substrate {
                    current_substrate.push(id);
                } else {
                    current_product.push(id);
                }
            }
        } else if line.starts_with(":precondition") {
            reading_substrate = true;
        } else if line.starts_with(":effect") {
            reading_substrate = false;
        }
    }

    trace!("Saving last reaction: {}", current_name);
    let mut reac = Reaction::new(reaction_counter, current_name.clone());
    for sub in &current_substrate {
        reac.add_substrate(sub.clone());
    }

    for prod in &current_product {
        reac.add_product(prod.clone());
    }

    pathway.add_reaction(reac);

    pathway
}
