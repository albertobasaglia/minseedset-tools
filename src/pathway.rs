#[derive(Debug)]
pub struct Compound {
    // Internal ID
    id: u32,
    // ID used in the file
    name: u32,
}

impl Compound {
    pub fn new(id: u32, name: u32) -> Self {
        Compound { id, name }
    }
}

#[derive(Debug)]
pub struct Reaction {
    // Internal ID
    id: u32,
    // ID used in the file
    name: u32,
    // Contains the IDs
    substrate: Vec<u32>,
    // Contains the IDs
    product: Vec<u32>,
}

impl Reaction {
    pub fn new(id: u32, name: u32) -> Self {
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

    pub fn get_compound_id(&self, name: u32) -> u32 {
        self.compounds
            .iter()
            .find(|x| x.name == name)
            .map(|x| x.id)
            .unwrap()
    }

    pub fn add_compound(&mut self, compound: Compound) {
        self.compounds.push(compound);
    }

    pub fn add_reaction(&mut self, reaction: Reaction) {
        self.reactions.push(reaction);
    }
}
