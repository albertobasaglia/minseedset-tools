use std::collections::HashSet;

use crate::pw::compound::Compound;
use crate::pw::reaction::Reaction;
use log::debug;
use serde::{Deserialize, Serialize};

/// Struct per rappresentare l'insieme di reazioni e di molecole all'interno
/// di un organismo
#[derive(Serialize, Deserialize, Debug)]
pub struct Pathway {
    /// Insieme delle molecole
    compounds: Vec<Compound>,

    /// Insieme delle reazioni
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

        // Pop all the reactions
        while let Some(mut reaction) = self.reactions.pop() {
            let mut dup = false;
            for ins in &new_reactions {
                if reaction.has_same_product(&ins) && reaction.has_same_substrate(&ins) {
                    debug!("Removing {:?} ------ duplicate of {:?}", reaction, ins);
                    dup = true;
                    dup_count += 1;
                    break;
                }
            }
            // insert them in the new vector if they aren't duplicated
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
    pub fn join_dominated_product(&mut self) -> u32 {
        let mut new_reactions: Vec<Reaction> = vec![];
        let mut dom_count = 0;
        let mut id_counter = 0;

        for i in 0..self.reactions.len() {
            let mut is_dominated = false;
            let reaction_i = &self.reactions[i];

            for j in 0..self.reactions.len() {
                let reaction_j = &self.reactions[j];

                if reaction_i.has_same_substrate(reaction_j)
                    && reaction_i.is_product_subset(reaction_j)
                {
                    debug!(
                        "Removing {:?} ------ Product dominated by {:?}",
                        reaction_i, reaction_j
                    );
                    is_dominated = true;
                    break;
                }
            }

            if is_dominated {
                dom_count += 1;
            } else {
                let mut reac = reaction_i.clone();
                reac.id = id_counter;
                id_counter += 1;
                new_reactions.push(reac);
            }
        }

        self.reactions = new_reactions;

        dom_count
    }

    /// WARNING: This changes the IDs of the reactions!
    pub fn join_dominated_substrate(&mut self) -> u32 {
        let mut new_reactions: Vec<Reaction> = vec![];
        let mut dom_count = 0;
        let mut id_counter = 0;

        for i in 0..self.reactions.len() {
            let mut is_dominated = false;
            let reaction_i = &self.reactions[i];

            for j in 0..self.reactions.len() {
                let reaction_j = &self.reactions[j];

                if reaction_i.has_same_product(reaction_j)
                    && reaction_j.is_substrate_subset(reaction_i)
                {
                    debug!(
                        "Removing {:?} ------ Substrate dominated by {:?}",
                        reaction_i, reaction_j
                    );
                    is_dominated = true;
                    break;
                }
            }

            if is_dominated {
                dom_count += 1;
            } else {
                let mut reac = reaction_i.clone();
                reac.id = id_counter;
                id_counter += 1;
                new_reactions.push(reac);
            }
        }

        self.reactions = new_reactions;

        dom_count
    }
    /// WARNING: This changes the IDs of the reactions!
    pub fn merge_reactions(&mut self) -> u32 {
        let mut new_reactions: Vec<Reaction> = vec![];
        let mut dup_count = 0;

        let mut id_counter = 0;

        let mut used = HashSet::<u32>::new();
        while let Some(mut reaction) = self.reactions.pop() {
            let mut dup = false;
            if used.contains(&reaction.id) {
                continue;
            }
            for ins in &self.reactions {
                if used.contains(&ins.id) {
                    continue;
                }
                // I have to find a reaction with the same substrate
                if reaction.has_same_substrate(&ins) {
                    used.insert(ins.id);
                    let mut new_reac = Reaction::new(id_counter, "merged".to_string());
                    id_counter += 1;
                    for sub in &reaction.substrate {
                        new_reac.add_substrate(sub.clone());
                    }
                    for prod in &reaction.product {
                        new_reac.add_product(prod.clone());
                    }
                    for prod in &ins.product {
                        if !new_reac.product.contains(prod) {
                            new_reac.product.push(prod.clone());
                        }
                    }
                    debug!("Merging {:?} and {:?} into {:?}", reaction, ins, new_reac);
                    new_reactions.push(new_reac);
                    dup_count += 1;
                    dup = true;
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
