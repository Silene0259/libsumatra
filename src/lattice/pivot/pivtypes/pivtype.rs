/// PivotType
pub enum PivType {
    Default(u16),
    Authority(PivTypeAuthority),
    DistributedDatabase()
}

pub enum PivTypeAuthority {
    IdAssigner,
    TrustAssigner,
}

pub enum PivTypeDistributedDatabase {
    Pivots, // needs to be on-chain
    KeyStorage, // needs to be on-chain
    AddressStorage, // needs to be on-chain
    NodeStorage, // needs to off-chain and distributed
}