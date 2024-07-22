pub struct PivotPublicKey(String);

pub struct PivotSignature(String);

impl PivotPublicKey {
    pub fn to_string(&self) -> String {
        return self.0
    }
}

impl PivotSignature {
    pub fn to_string(&self) -> String {
        return self.0
    }
}