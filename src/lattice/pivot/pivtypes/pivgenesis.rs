use crate::lattice::pivot::pivot::PivotInit;

use serde::{Serialize,Deserialize};

#[derive(Clone, Serialize,Deserialize)]
pub struct GenesisPivotHash(String);

impl GenesisPivotHash {
    pub fn new<T: AsRef<str>>(pivot: PivotInit, key: T) -> Self {
        Self(pivot.get_pivgenesishash(key))
    }
}