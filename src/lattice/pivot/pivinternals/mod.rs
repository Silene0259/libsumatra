use crate::lattice::pivot::pivot::PivotInit;
use crate::lattice::pivot::pivtypes::nonce::Nonce;
use crate::lattice::pivot::pivtypes::pivgenesis::GenesisPivotHash;


pub struct GenesisPivotConfig {
    pivot: PivotInit,

    nonce: Nonce,
}