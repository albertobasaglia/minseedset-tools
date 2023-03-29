#[derive(Debug)]
pub struct Compound {
    // Internal ID
    id: u32,
    // ID used in the file
    name: String,
}

impl Compound {
    pub fn new(id: u32, name: String) -> Self {
        Compound { id, name }
    }
}

#[derive(Debug)]
pub struct Reaction {
    // Internal ID
    id: u32,
    // ID used in the file
    name: String,
    // Contains the IDs
    substrate: Vec<u32>,
    // Contains the IDs
    product: Vec<u32>,
}

impl Reaction {
    pub fn new(id: u32, name: String) -> Self {
        Reaction {
            id,
            name,
            substrate: vec![],
            product: vec![],
        }
    }

    pub fn add_substrate(&mut self, id: u32) {
        self.substrate.push(id);
    }

    pub fn add_product(&mut self, id: u32) {
        self.product.push(id);
    }

    pub fn get_substrate(&self) -> &Vec<u32> {
        &self.substrate
    }

    pub fn get_product(&self) -> &Vec<u32> {
        &self.product
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn is_substrate_subset(&self, other: &Self) -> bool {
        for s in &self.substrate {
            if !other.substrate.contains(s) {
                return false;
            }
        }
        true
    }

    pub fn is_product_subset(&self, other: &Self) -> bool {
        for p in &self.product {
            if !other.product.contains(p) {
                return false;
            }
        }
        true
    }

    pub fn has_same_substrate(&self, other: &Self) -> bool {
        if !self.is_substrate_subset(other) {
            return false;
        }
        if !other.is_substrate_subset(self) {
            return false;
        }
        true
    }

    pub fn has_same_product(&self, other: &Self) -> bool {
        if !self.is_product_subset(other) {
            return false;
        }
        if !other.is_product_subset(self) {
            return false;
        }
        true
    }
}

#[derive(Debug)]
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
                if reaction.has_same_product(&ins) && reaction.is_substrate_subset(&ins) {
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
