/// 40-byte string using BLAKE2B(40) that can be keyed with secret value
pub struct PivFingerprint(String);

impl PivFingerprint {
    pub fn to_string(&self) -> String {
        return self.0
    }
}