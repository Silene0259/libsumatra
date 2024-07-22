use crate::lattice::pivot::pivot::PivotInit;
use serde::{Serialize,Deserialize};
use serde_json;

#[derive(Serialize,Deserialize)]
pub struct GenesisBlockLattice {
    pivot: PivotInit,
    hash: String,

}

impl GenesisBlockLattice {
    pub fn new(pivot: PivotInit) -> Self {
        Self {
            pivot: pivot,
            hash: String::from("TestString")
        }
    }
    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(&self).expect("Failed To Convert To JSON")
    }
}