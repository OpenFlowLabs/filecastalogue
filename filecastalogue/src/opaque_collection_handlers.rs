use std::{ffi::{OsStr, OsString}, path::{Path, PathBuf}};
use crate::{error::{Error, ErrorKind, FcResult, KeyValuePayload},
finite_stream_handlers::FiniteStreamHandler};

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
    fn get_deabsolutized_file_name<NameRef: AsRef<OsStr>>
    (self: &mut Self, name: NameRef) -> FcResult<OsString> {
        let file_name_path = PathBuf::from(name.as_ref());
        match file_name_path.file_name() {
            Some(file_name) => Ok(file_name.to_owned()),
            None => Err(error!(
                ErrorKind::DoubleDotFileName,
                "Getting file_name portion of a path to make sure it isn't absolute.",
                payload => Some(
                    Box::new(
                        KeyValuePayload::new()
                        .add("Original path", Box::new(file_name_path.to_owned()))
                    )
                )
            ))
        }
    }

    fn get_file_path<NameRef: AsRef<OsStr>>(self: &mut Self, name: NameRef)
    -> FcResult<PathBuf> {
        let file_path = PathBuf::from(&self.path);
        match self.get_deabsolutized_file_name(name) {
            Ok(file_name) => Ok(file_path.join(file_name)),
            Err(e) => Err(e)
        }
    }

    // fn get_file<NameRef: AsRef<OsString>>(self: &mut Self, name: NameRef)
    // -> FcResult<File> {
    //     let localized_name = PathBuf::from(name.as_ref());
    //     File::open(self.path.join(localized_name))
    // }   
}

// A collection of files of which we know nothing except that
// it holds an unknown number (incl. 0) of files of a certain kind.
pub trait OpaqueCollectionHandler {
    fn has_file(self: &mut Self, name: &OsStr) -> bool;
    fn get_file<T>(self: &mut Self, name: &OsStr)
    -> FcResult<T> where T: FiniteStreamHandler;
    // TODO: Wrap in proper error. What we get here doesn't necessarily
    // have to be io::Error.
    // fn read_file<T, NameRef: AsRef<OsStr>>(self: &mut Self, name: NameRef)
    // -> FcResult<T>;
}

impl OpaqueCollectionHandler for LocalDir
{
    fn has_file(self: &mut Self, name: &OsStr) -> bool {
        self.path.exists()
    }

    fn get_file<T>(self: &mut Self, name: &OsStr)
    -> FcResult<T> where T: FiniteStreamHandler {
        let path = self.get_file_path(name)?;
        Ok(FiniteStreamHandler::new(path))
    }

    // fn read_file<T, NameRef: AsRef<OsStr>>(self: &mut Self, name: NameRef)
    // -> FcResult<T> {
    //     // let file = self.get_file(name)?;
    //     let file_path = self.get_file_path(name)?;
    //     IndexFile::new(LocalFile::new(file_path))
    // }
}