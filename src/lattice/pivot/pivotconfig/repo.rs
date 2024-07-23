use git2::Repository;
use std::path::Path;

pub struct SumatraRepo(Repository);

impl SumatraRepo {
    pub fn open<T: AsRef<Path>>(path: T) -> Self {
        Self(Repository::open(path.as_ref()).expect("Failed To Open Path"))
    }
    pub fn init<T: AsRef<Path>>(path: T) -> Self {
        Self(Repository::init(path.as_ref()).expect("Failed To Unwrap Repository"))
    }
}