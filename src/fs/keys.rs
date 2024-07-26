use crate::fs::storage::SumatraHomeDirectoryKeyspace;
use std::path::{Path,PathBuf};
use std::fs;

pub const PERSONALKEYS: &str = "userkeys";

pub struct SumatraPersonalKeysDir(PathBuf);


impl SumatraHomeDirectoryKeyspace {
    pub fn init_personal_keys(&self) -> SumatraPersonalKeysDir {
        let keyspace_dir = &self.return_keyspace();
        let mut current_path = keyspace_dir.to_owned();
        
        current_path.push(Path::new(PERSONALKEYS));

        fs::create_dir(&current_path);

        let final_path = current_path.clone();

        let sumatraconfig = SumatraPersonalKeysDir(final_path);
        return sumatraconfig
    }
}