use serde::{Serialize,Deserialize};

#[derive(Clone, Serialize,Deserialize)]
pub enum GeneralPivotRules {
    General,
}