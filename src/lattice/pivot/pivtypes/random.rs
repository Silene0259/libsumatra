use serde::{Serialize,Deserialize};

#[derive(Serialize,Deserialize)]
/// CSPRNG
pub struct RandomNumbers(String);

impl RandomNumbers {
    pub fn new<T: AsRef<str>>(random: T) -> Self {
        return Self(random.as_ref().to_string())
    }
}

/// Quantum-RNG from Site
pub struct QuantumRandomNumbers(String);