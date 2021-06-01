use std::{ffi::{OsStr, OsString}, fs::File, io, path::{Path, PathBuf}};

use crate::{error::{Error, FcResult}, files::index::drivers::local::IndexFile, finite_stream_handlers::LocalFile};

pub struct LocalDir {
    path: PathBuf
}

impl LocalDir {
    pub fn new<PathRef: AsRef<Path>>(path: PathRef) -> Self {
        Self {
            path: path.as_ref().to_owned()
        }
    }

    // Making sure our file name isn't absolute, to prevent
    // accidentally replacing the base directory path in
    // .join operations.
    fn get_deabsolutized_file_name<NameRef: AsRef<OsString>>
    (self: &mut Self, name: NameRef) -> FcResult<&OsStr> {
        let file_name_path = PathBuf::from(name.as_ref());
        match file_name_path.file_name() {
            Some(file_name) => file_name,
            None => Error::new(
                ErrorKind
            )
        }
    }

    fn make_file_path<NameRef: AsRef<OsString>>(self: &mut Self, name: NameRef)
    -> FcResult<PathBuf> {
        let file_path = PathBuf::from(self.path);
        match self.get_deabsolutized_file_name(name) {
            Some(file_name) => file_path.join(file_name),
            None => 
        }
    }

    fn get_file<NameRef: AsRef<OsString>>(self: &mut Self, name: NameRef)
    -> Result<File, io::Error> {
        let localized_name = PathBuf::from(name);
        File::open(self.path.join(localized_name))
    }    
}

// A collection of files of which we know nothing except that
// it holds an unknown number (incl. 0) of files of a certain kind.
pub trait OpaqueCollectionHandler {
    fn has_file(self: &mut Self, name: &OsString) -> bool;
    // TODO: Wrap in proper error. What we get here doesn't necessarily
    // have to be io::Error.
    fn read_file<T>(self: &mut Self, name: &OsString) -> Result<T, io::Error>;
}

impl OpaqueCollectionHandler for LocalDir
{
    fn has_file(self: &mut Self, name: &OsString) -> bool {
        self.path.exists()
    }

    fn read_file<T>(self: &mut Self, name: &OsString) -> Result<T, io::Error> {
        let file = self.get_file(name)?;
        IndexFile::new(LocalFile::new());
    }
}