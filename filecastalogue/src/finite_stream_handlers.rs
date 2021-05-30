use std::{fs::File, io::{self, BufReader}, path::{Path, PathBuf}};

use serde::de::{DeserializeOwned};

pub struct LocalFile {
    path: PathBuf
}

impl LocalFile {
    pub fn new<PathRef: AsRef<Path>>(path: PathRef) -> Self {
        Self {
            path: path.as_ref().to_owned()
        }
    }
}

pub trait FiniteStreamHandler {
    fn read_all<Target>(self: &mut Self) -> Result<Target, io::Error>
    where Target: DeserializeOwned;
    fn write_all();
}

impl FiniteStreamHandler for LocalFile {
    fn read_all<Target>(self: &mut Self) -> Result<Target, io::Error>
    where Target: DeserializeOwned {
        Ok(serde_json::from_reader(BufReader::new(File::open(self.path.to_owned())?))?)
    }

    fn write_all() {
        todo!()
    }
}