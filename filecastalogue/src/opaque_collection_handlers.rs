use std::{ffi::OsString, io, path::{Path, PathBuf}};

pub struct LocalDir {
    path: PathBuf
}

impl LocalDir {
    pub fn new<PathRef: AsRef<Path>>(path: PathRef) -> Self {
        Self {
            path: path.as_ref().to_owned()
        }
    }
}

// A collection of files of which we know nothing except that
// it holds an unknown number (incl. 0) of files of a certain kind.
pub trait OpaqueCollectionHandler {
    fn has_file(name: &OsString) -> Result<bool, io::Error>;
    fn get_file<T>(name: &OsString) -> Result<T, io::Error>;
}

impl OpaqueCollectionHandler for LocalDir {
    fn has_file(name: &OsString) -> Result<bool, io::Error> {
        todo!()
    }

    fn get_file<T>(name: &OsString) -> Result<T, io::Error> {
        todo!()
    }
}