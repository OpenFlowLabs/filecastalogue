use std::{fs::File, io::{self, BufReader, BufWriter}, path::{Path, PathBuf}};

use serde::{Serialize, de::{DeserializeOwned}};

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
    fn write_all<Source>(self: &mut Self, source: &Source) -> Result<(), io::Error>
    where Source: ?Sized + Serialize;
}

impl FiniteStreamHandler for LocalFile {
    fn read_all<Target>(self: &mut Self) -> Result<Target, io::Error>
    where Target: DeserializeOwned {
        Ok(serde_json::from_reader(
            BufReader::new(File::open(self.path.to_owned())?)
        )?)
    }

    fn write_all<Source>(self: &mut Self, source: &Source) -> Result<(), io::Error>
    where Source: ?Sized + Serialize {
        Ok(serde_json::to_writer_pretty(
            BufWriter::new(File::open(self.path.to_owned())?),
            source
        )?)
    }
}