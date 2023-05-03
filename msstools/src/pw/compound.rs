use serde::{Deserialize, Serialize};

/// Struct per rappresentare un elemento in un organismo
#[derive(Serialize, Deserialize, Debug)]
pub struct Compound {
    /// ID interno
    pub id: u32,

    /// ID letto dal file
    pub name: String,
}

impl Compound {
    pub fn new(id: u32, name: String) -> Self {
        Compound { id, name }
    }
}
