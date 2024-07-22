use libsumatracrypt_rs::digest::SumatraBlake2b;

use crate::lattice::pivot::pivtypes::*;

// Need Digital Signature

pub struct PivotInit {
    version: version::PivotVersion,
    
    pk: publickey::PivotPublicKey,

    // RNG
    csprng: random::RandomNumbers,
    qrng: random::QuantumRandomNumbers,

    pivottype: pivtype::PivType,
    rules: pivotrules::GeneralPivotRules,

    // Hash All Above And Return As Fingerprint
    fingerprinthash: fingerprint::PivFingerprint,

    // Sign Fingerprint
    signature: publickey::PivotSignature,
}

impl PivotInit {
    pub fn new() {
        
    }
    pub fn get_pivgenesishash<T: AsRef<str>>(&self, key: T) -> String {
        let mut result = String::new();
        
        let fingerprint = self.fingerprinthash.to_string();
        let pk = self.pk.to_string();
        let signature = self.signature.to_string();

        result.push_str(&fingerprint);
        result.push_str(&pk);
        result.push_str(&signature);
        
        return SumatraBlake2b::new(result, key.as_ref().to_string(), 40usize);
    }
}

pub struct PivotInternals;