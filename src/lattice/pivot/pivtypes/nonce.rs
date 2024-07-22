use serde::{Serialize,Deserialize};

#[derive(Copy, Clone, Serialize,Deserialize)]
pub struct Nonce(u64);