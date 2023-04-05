#[derive(Debug)]
pub struct Reaction {
    // Internal ID
    pub id: u32,
    // ID used in the file
    pub name: String,
    // Contains the IDs
    pub substrate: Vec<u32>,
    // Contains the IDs
    pub product: Vec<u32>,
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
