use serde::{Deserialize, Serialize};

/// Struct per rappresentare una reazione all'interno di un organismo
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Reaction {
    /// ID usato all'interno dell'organismo
    pub id: u32,

    /// ID letto dal file
    pub name: String,

    /// Insieme dei reagenti
    pub substrate: Vec<u32>,

    /// Insieme dei prodotti
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
        if self.has_same_substrate(other) {
            return false;
        }
        for s in &self.substrate {
            if !other.substrate.contains(s) {
                return false;
            }
        }
        true
    }

    pub fn is_product_subset(&self, other: &Self) -> bool {
        if self.has_same_product(other) {
            return false;
        }
        for p in &self.product {
            if !other.product.contains(p) {
                return false;
            }
        }
        true
    }

    pub fn is_product_subset_or_superset(&self, other: &Self) -> bool {
        self.is_product_subset(other) || other.is_product_subset(self)
    }

    pub fn is_substrate_subset_or_superset(&self, other: &Self) -> bool {
        self.is_substrate_subset(other) || other.is_substrate_subset(self)
    }

    pub fn has_same_substrate(&self, other: &Self) -> bool {
        let mut vec1 = self.substrate.to_vec();
        let mut vec2 = other.substrate.to_vec();
        vec1.sort();
        vec2.sort();
        return vec1 == vec2;
    }

    pub fn has_same_product(&self, other: &Self) -> bool {
        let mut vec1 = self.product.to_vec();
        let mut vec2 = other.product.to_vec();
        vec1.sort();
        vec2.sort();
        return vec1 == vec2;
    }
}
