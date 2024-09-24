use libsumatracrypt_rs::signatures::ed25519::{ED25519PublicKey,ED25519Signature,ED25519SecretKey};
use libsumatracrypt_rs::encryption::{SumatraEncryptECIES,ECIESPublicKey,ECIESSecretKey,ECIESDecodedMessage,ECIESCipherText};

use libsumatracrypt_rs::digest::SumatraBlake2b;

pub struct SlabUser {
    pk: ED25519PublicKey,
    pk_enc: ECIESPublicKey,
    sig_of_enc: ED25519Signature,
}

pub struct SlabUserSecrets {
    sk: ED25519SecretKey,
    sk_enc: ECIESSecretKey,
}

pub struct Slab {
    email: String,
    phone: String,
}