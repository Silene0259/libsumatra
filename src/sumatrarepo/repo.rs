use git2::Repository;
use git2::Signature;
use libsumatracrypt_rs::signatures::ed25519::ED25519PublicKey;
use libsumatracrypt_rs::signatures::ed25519::ED25519SecretKey;

use libsumatracrypt_rs::csprng::SumatraCSPRNG;
use hex;

use std::path::Path;

pub struct SumatraRepo {
    repo: Repository,
    
    csprng: String,
    pk: String,
    signature: String,
}

pub struct SumatraRepoSignature(Signature);

impl SumatraRepo {
    pub fn commit(&self) -> Self {

    }
    pub fn open<T: AsRef<Path>>(path: T) -> Self {
        Self(Repository::open(path.as_ref()).expect("Failed To Open Path"))
    }
    pub fn init<T: AsRef<Path>>(path: T, pk: ED25519PublicKey, sk: ED25519SecretKey) -> Self {
        let csprng = hex::encode_upper(SumatraCSPRNG::new_32());
        let publickey = pk.to_string();

        git2::Cred::credtype(&self)
        
        Self(Repository::init(path.as_ref()).expect("Failed To Unwrap Repository"))
    }
}

impl SumatraRepoSignature {
    pub fn new<T: AsRef<str>>(author: T, email: T) {
        
        
        Signature::new(author.as_ref(), email.as_ref(), time)
    }
}