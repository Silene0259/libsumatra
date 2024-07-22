use libsumatracrypt_rs::signatures::ed25519::*;

use serde::{Serialize,Deserialize};

#[derive(Clone,Serialize,Deserialize)]
pub struct PivotPublicKey(ED25519PublicKey);

#[derive(Clone,Serialize,Deserialize)]
pub struct PivotSecretKey(ED25519SecretKey);

#[derive(Clone,Serialize,Deserialize)]
pub struct PivotSignature(ED25519Signature);

impl PivotPublicKey {
    pub fn from_public_key(pk: ED25519PublicKey) -> Self {
        return Self(pk)
    }
    pub fn to_string(&self) -> String {
        return self.0.to_string()
    }
}

impl PivotSignature {
    pub fn to_string(&self) -> String {
        return self.0.to_string()
    }
}

impl PivotSecretKey {
    pub fn from_secret_key(sk: ED25519SecretKey) -> Self {
        return Self(sk)
    }
    pub fn to_string(&self) -> String {
        return self.0.to_string()
    }
    pub fn sign<T: AsRef<[u8]>>(&self, data: T) -> PivotSignature {
        PivotSignature(self.0.sign(data))
    }
}