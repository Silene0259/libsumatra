use dirs::home_dir;
use std::{ffi::OsStr, path::{Path,PathBuf}};
use std::fs;

pub const SUMATRANAME: &str = ".sumatra";
pub const CONFIGNAME: &str = "config";
pub const KEYSPACE: &str = "keyspace";

/// Home Directory `$HOMEDIR/.sumatra`
pub struct SumatraHomeDirectory(PathBuf);

/// Config Location `(.sumatra/config/)`
pub struct SumatraHomeDirectoryConfig(PathBuf);

/// Keyspace Location `(.sumatra/keyspace/)``
pub struct SumatraHomeDirectoryKeyspace(PathBuf);


pub struct KeyspacePersonalKeys(PathBuf);

impl SumatraHomeDirectory {
    pub fn init_home() -> SumatraHomeDirectory {
        let home_dir = home_dir().expect("Failed To Get Home Dir");
        let mut current_path = home_dir;


        current_path.push(Path::new(SUMATRANAME));

        fs::create_dir(current_path.clone());

        let sumatra_home_directory = SumatraHomeDirectory(current_path.to_owned());
        return sumatra_home_directory
    }
    pub fn init_config(&self) -> SumatraHomeDirectoryConfig {
        let home_dir = &self.0;
        let mut current_path = home_dir.to_owned();
        
        current_path.push(Path::new(CONFIGNAME));

        fs::create_dir(&current_path);

        let final_path = current_path.clone();

        let sumatraconfig = SumatraHomeDirectoryConfig(final_path);
        return sumatraconfig
    }
    pub fn init_keyspace(&self) -> SumatraHomeDirectoryKeyspace {
        let mut home_dir = &self.0;
        let mut current_path = home_dir.to_owned();
        
        current_path.push(Path::new(KEYSPACE));

        fs::create_dir(&current_path);

        let final_path = current_path.clone();

        let sumatrakeyspace = SumatraHomeDirectoryKeyspace(final_path);
        return sumatrakeyspace
    }
}

impl SumatraHomeDirectoryKeyspace {
    pub fn return_keyspace(&self) -> PathBuf {
        return self.0.clone()
    }
}

#[test]
fn create_file() {
    SumatraHomeDirectory::init_home();
}