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
}
