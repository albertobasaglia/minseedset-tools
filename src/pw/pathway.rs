use crate::pw::compound::Compound;
use crate::pw::reaction::Reaction;
use log::debug;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Pathway {
    compounds: Vec<Compound>,
    reactions: Vec<Reaction>,
}

impl Pathway {
    pub fn new() -> Self {
        Pathway {
            compounds: vec![],
            reactions: vec![],
        }
    }

    pub fn get_compound_id(&self, name: &String) -> u32 {
        self.compounds
            .iter()
            .find(|x| &x.name == name)
            .map(|x| x.id)
            .unwrap()
    }

    pub fn get_compound_option(&self, name: &String) -> Option<u32> {
        self.compounds
            .iter()
            .find(|x| &x.name == name)
            .map(|x| x.id)
    }

    pub fn add_compound(&mut self, compound: Compound) {
        self.compounds.push(compound);
    }

    pub fn add_reaction(&mut self, reaction: Reaction) {
        self.reactions.push(reaction);
    }

    pub fn get_compounds_count(&self) -> usize {
        self.compounds.len()
    }

    pub fn get_reactions_count(&self) -> usize {
        self.reactions.len()
    }

    pub fn get_reactions(&self) -> &Vec<Reaction> {
        &self.reactions
    }

    /// WARNING: This changes the IDs of the reactions!
    pub fn split_multiple_product(&mut self) -> u32 {
        let mut reaction_counter = 0;
        let mut new_reactions: Vec<Reaction> = vec![];
        let mut split_count = 0;
        for reaction in &self.reactions {
            if reaction.get_product().len() > 1 {
                split_count += 1;
            }
            for product in reaction.get_product() {
                let mut new_reac = Reaction::new(
                    reaction_counter,
                    format!("{}_{}", reaction.get_name(), product.to_string()),
                );
                reaction_counter += 1;
                for substrate in reaction.get_substrate() {
                    new_reac.add_substrate(substrate.clone());
                }
                new_reac.add_product(product.clone());
                new_reactions.push(new_reac);
            }
        }
        self.reactions = new_reactions;
        split_count
    }

    /// WARNING: This changes the IDs of the reactions!
    pub fn join_duplicates(&mut self) -> u32 {
        let mut new_reactions: Vec<Reaction> = vec![];
        let mut dup_count = 0;

        let mut id_counter = 0;

        while let Some(mut reaction) = self.reactions.pop() {
            let mut dup = false;
            for ins in &new_reactions {
                if reaction.has_same_product(&ins) && reaction.has_same_substrate(&ins) {
                    dup = true;
                    dup_count += 1;
                    break;
                }
            }
            if !dup {
                reaction.id = id_counter;
                new_reactions.push(reaction);
                id_counter += 1;
            }
        }

        self.reactions = new_reactions;
        dup_count
    }

    /// WARNING: This changes the IDs of the reactions!
    pub fn join_dominated(&mut self) -> u32 {
        let mut new_reactions: Vec<Reaction> = vec![];
        let mut dup_count = 0;

        let mut id_counter = 0;

        while let Some(mut reaction) = self.reactions.pop() {
            let mut dup = false;
            for ins in &self.reactions {
                if reaction.has_same_substrate(&ins) && reaction.is_product_subset(&ins) {
                    debug!("Removing {:?} ------ Dominated by {:?}", reaction, ins);
                    dup = true;
                    dup_count += 1;
                    break;
                }
            }
            if !dup {
                reaction.id = id_counter;
                new_reactions.push(reaction);
                id_counter += 1;
            }
        }

        self.reactions = new_reactions;
        dup_count
    }
}
