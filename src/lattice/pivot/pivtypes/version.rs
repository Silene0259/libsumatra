use serde::{Serialize,Deserialize};

#[derive(Clone,Copy,Serialize,Deserialize)]
pub enum PivotVersion {
    V0000,
}