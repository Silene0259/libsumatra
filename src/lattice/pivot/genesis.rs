use crate::lattice::pivot::pivot::PivotInit;
use crate::lattice::pivot::pivtypes::keys::*;
use crate::lattice::pivot::pivotconfig::configure::PivotConfig;

use libsumatracrypt_rs::signatures::ed25519::ED25519Signature;
use libsumatracrypt_rs::digest::SumatraBlake2b;

use serde::{Serialize,Deserialize};
use serde_json;

// Genesis Pivot features multiple versions per pivot to advocate for changes in network.

#[derive(Serialize,Deserialize)]
pub struct GenesisPivotPoint {
    pivot: PivotInit,

    pivotconfig: PivotConfig,
    nonce: u64,
    hash: String,
    signature: ED25519Signature,
}

impl GenesisPivotPoint {
    pub fn new(sk: PivotSecretKey, pivot: PivotInit, pivot_config: PivotConfig) -> Self {
        //let hash = SumatraBlake2b::new(bytes, key, digest_size)
        
        //let sig = sk.sign();
        Self {
            pivot: pivot,
            pivotconfig: pivot_config,
            nonce: 0u64,
            hash: String::from("TestString"),
            signature: todo!(),
        }
    }
    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).expect("Failed To Convert To JSON")
    }
}