use serde::{Serialize,Deserialize};

/// PivotType
/// 

#[derive(Serialize,Deserialize)]
pub enum PivType {
    Default(u16),
    Authority(PivTypeAuthority),
    DistributedDatabase()
}

#[derive(Serialize,Deserialize)]
pub enum PivTypeAuthority {
    IdAssigner,
    TrustAssigner,
}

#[derive(Serialize,Deserialize)]
pub enum PivTypeDistributedDatabase {
    Pivots, // needs to be on-chain
    KeyStorage, // needs to be on-chain
    AddressStorage, // needs to be on-chain
    NodeStorage, // needs to off-chain and distributed
}