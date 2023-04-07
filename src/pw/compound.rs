use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Compound {
    // Internal ID
    pub id: u32,
    // ID used in the file
    pub name: String,
}

impl Compound {
    pub fn new(id: u32, name: String) -> Self {
        Compound { id, name }
    }
}
