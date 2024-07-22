use fingerprint::PivFingerprint;
use libsumatracrypt_rs::digest::SumatraBlake2b;
use hex;

use serde::{Serialize,Deserialize};

use crate::lattice::pivot::pivtypes::*;

// Need Digital Signature

#[derive(Serialize,Deserialize)]
pub struct PivotInit {
    version: version::PivotVersion,
    
    pk: keys::PivotPublicKey,

    // RNG
    csprng: random::RandomNumbers,

    pivottype: pivtype::PivType,
    rules: pivotrules::GeneralPivotRules,

    // Hash All Above And Return As Fingerprint
    fingerprinthash: fingerprint::PivFingerprint,

    // Sign Fingerprint
    signature: keys::PivotSignature,
}

#[derive(Serialize,Deserialize)]
pub struct PivotToHash {
    version: version::PivotVersion,
    pk: keys::PivotPublicKey,
    csprng: random::RandomNumbers,
    pivottype: pivtype::PivType,
    rules: pivotrules::GeneralPivotRules,

}

impl PivotInit {
    pub fn new(pk: keys::PivotPublicKey, sk: keys::PivotSecretKey, pivottype: pivtype::PivType, rules: pivotrules::GeneralPivotRules) -> Self {
        let version = version::PivotVersion::V0000;
        let csprng = random::RandomNumbers::new(hex::encode_upper(libsumatracrypt_rs::csprng::SumatraCSPRNG::get_64_bytes_from_os()));

        let pivtohash = PivotToHash {
            version: version,
            pk: pk,
            csprng: csprng,
            pivottype: pivottype,
            rules: rules,
        };

        let serialized = serde_json::to_string(&pivtohash).expect("Failed To Convert");

        let key = String::from("");
        let fingerprinthash = SumatraBlake2b::new(&serialized, &key, 40);
        let pivfingerprint = PivFingerprint::new(fingerprinthash);

        let signed = sk.sign(fingerprinthash);

        return Self {
            version: version,
            pk: pk,
            csprng: csprng,
            pivottype: pivottype,
            rules: rules,
            fingerprinthash: pivfingerprint,
            signature: signed,
        }
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