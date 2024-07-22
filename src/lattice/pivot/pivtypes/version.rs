use serde::{Serialize,Deserialize};

#[derive(Serialize,Deserialize)]
pub enum PivotVersion {
    V0000,
}