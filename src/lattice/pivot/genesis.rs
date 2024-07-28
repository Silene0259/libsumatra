use crate::lattice::pivot::pivot::PivotInit;

use libsumatracrypt_rs::signatures::ed25519::ED25519Signature;

use serde::{Serialize,Deserialize};
use serde_json;

// Genesis Pivot features multiple versions per pivot to advocate for changes in network.

#[derive(Serialize,Deserialize)]
pub struct GenesisPivotPoint {
    pivot: PivotInit,
    hash: String,
    signature: ED25519Signature,
}

impl GenesisPivotPoint {
    pub fn new(pivot: PivotInit) -> Self {
        
        Self {
            pivot: pivot,
            hash: String::from("TestString"),
            signature: todo!(),
        }
    }
    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(&self).expect("Failed To Convert To JSON")
    }
}