use serde::{Serialize,Deserialize};

/// 40-byte string using BLAKE2B(40) that can be keyed with secret value
#[derive(Clone,PartialEq,Serialize,Deserialize)]
pub struct PivFingerprint(String);

impl PivFingerprint {
    pub fn new<T: AsRef<str>>(fingerprint: T) -> Self {
        return Self(fingerprint.as_ref().to_string())
    }
    pub fn to_string(&self) -> String {
        return self.0.to_string()
    }
}