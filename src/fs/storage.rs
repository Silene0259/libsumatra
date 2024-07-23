use dirs::home_dir;
use std::{ffi::OsStr, path::{Path,PathBuf}};
use std::fs;

pub const SUMATRANAME: &str = ".sumatra";

pub struct SumatraHomeDirectory(Path);

impl SumatraHomeDirectory {
    pub fn init() {
        let home_dir = home_dir().expect("Failed To Get Home Dir");
        let mut current_path = home_dir;


        current_path.push(Path::new(SUMATRANAME));

        fs::create_dir(current_path);
    }
}

#[test]
fn create_file() {
    SumatraHomeDirectory::init();
}