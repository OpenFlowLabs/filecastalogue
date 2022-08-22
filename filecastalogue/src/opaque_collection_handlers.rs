use std::{ffi::{OsStr, OsString}, fmt::Display, fs::{File, OpenOptions, create_dir}, io::{Read, Write}, path::{Path, PathBuf}};
use crate::{error::{Error, ErrorKind, ErrorPathBuf, FcResult, Payload}};

#[derive(Debug)]
pub struct PathDoesNotExistInCollectionPayload {
    pub collection_path: PathBuf,
    pub file_name: PathBuf
}
impl Display for PathDoesNotExistInCollectionPayload {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f, 
            concat!(
                "Collection path: {}, file name: {}.",
            ),
            ErrorPathBuf::from(self.collection_path.to_owned()),
            ErrorPathBuf::from(self.file_name.to_owned())
        )
    }
}
impl Payload for PathDoesNotExistInCollectionPayload {}

#[derive(Debug)]
pub struct DoubleDotFileName {
    pub original_path: PathBuf,
}
impl Display for DoubleDotFileName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Original path: {}",
            ErrorPathBuf::from(self.original_path.to_owned())
        )
    }
}
impl Payload for DoubleDotFileName {}

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
    (&self, name: NameRef) -> FcResult<OsString> {
        let file_name_path = PathBuf::from(name.as_ref());
        match file_name_path.file_name() {
            Some(file_name) => Ok(file_name.to_owned()),
            None => Err(error!(
                ErrorKind::DoubleDotFileName,
                "Getting file_name portion of a path to make sure it isn't absolute.",
                payload => DoubleDotFileName{
                    original_path: file_name_path.to_owned()
                }
            ))
        }
    }

    fn get_file_path<NameRef: AsRef<OsStr>>(&self, name: NameRef)
    -> FcResult<PathBuf> {
        let file_path = PathBuf::from(&self.path);
        match self.get_deabsolutized_file_name(name) {
            Ok(file_name) => Ok(file_path.join(file_name)),
            Err(e) => Err(e)
        }
    }

    fn get_file<NameRef: AsRef<OsStr>>(&self, name: NameRef, options: &mut OpenOptions) -> FcResult<File> {
        let path = self.get_file_path(name)?;
        match path.exists() {
            true => Ok(options.open(path.to_owned())?
            ),
            false => Err(error!(
                ErrorKind::PathDoesNotExistInCollection,
                "Getting a file from a LocalDir collection handler.",
                payload => PathDoesNotExistInCollectionPayload {
                    collection_path: self.path.to_owned(),
                    file_name: path.to_owned()
                }
            ))
        }
    }

    fn create_file<NameRef: AsRef<OsStr>>(&self, name: NameRef) -> FcResult<()> {
        let path = self.get_file_path(name)?;
        File::create(path)?;
        Ok(())
    }

    fn exists(&self) -> bool {
        self.path.exists()
    }

    /** Attempt to create the directory.
    */
    fn create(&self) -> FcResult<&Self> {
        create_dir(&self.path)?;
        Ok(self)
    }

    /** Attempt to create the directory and silently ignore it if it
        already exists.
        This will still pass on any other errors.
    */ 
    fn create_ignore_exists(self: &mut Self) -> FcResult<&mut Self> {
        let result = create_dir(&self.path);
        match result {
            Ok(_) => Ok(self),
            Err(e) => match e.kind() {
                std::io::ErrorKind::AlreadyExists => Ok(self),
                _ => Err(e.into())
            },
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
    fn has_file<NameRef: AsRef<OsStr>>(self: &mut Self, name: NameRef)
    -> FcResult<bool>;
    fn create_file<NameRef: AsRef<OsStr>>(&self,name: NameRef)
    -> FcResult<()>;
    fn get_file_readable(&self, name: &OsStr)
    -> FcResult<Box<(dyn Read)>>;
    fn get_file_writeable(&self, name: &OsStr)
    -> FcResult<Box<(dyn Write)>>;
    fn collection_exists(self: &mut Self) -> bool;
    fn create_collection(self: &mut Self) -> FcResult<()>;
    fn create_collection_ignore_exists(self: &mut Self) -> FcResult<()>;

    /// Returns a string containing info about the file the collection
    /// associates with the specified name.
    /// 
    /// This is intended to be used by error code on call sites that wouldn't
    /// normally have access to collection internals, in order to assemble
    /// more useful debug messages.
    /// 
    /// WARNING: This might not be secure for user facing errors. Particularly
    /// in distributed setups, carrying this output outside of the shell of the
    /// process where it was generated could, for example, leak URL embedded
    /// tokens used with network implementations of collections if they made
    /// that info available for debugging using this method.
    fn get_debug_info_for_file<NameRef: AsRef<OsStr>>(&self, name: NameRef) -> String;
}

impl OpaqueCollectionHandler for LocalDir
{
    fn has_file<NameRef: AsRef<OsStr>>(self: &mut Self, name: NameRef)
    -> FcResult<bool> {
        Ok(self.get_file_path(name)?.exists())
    }

    fn create_file<NameRef: AsRef<OsStr>>(&self, name: NameRef)
    -> FcResult<()> {
        self.create_file(name)?;
        // It's somewhat coincidental that our return value is the same
        // as that of `LocalDir::create_file`. It doesn't have to be.
        Ok(())
    }

    fn get_file_readable(&self, name: &OsStr)
    -> FcResult<Box<(dyn Read)>> {
        Ok(Box::new(self.get_file(name, OpenOptions::new().read(true))?))
    }

    fn get_file_writeable(&self, name: &OsStr)
    -> FcResult<Box<(dyn Write)>> {
        Ok(Box::new(self.get_file(name, OpenOptions::new().write(true))?))
    }

    fn collection_exists(self: &mut Self) -> bool {
        self.exists()
    }

    fn create_collection(self: &mut Self) -> FcResult<()> {
        self.create()?;
        Ok(())
    }

    fn create_collection_ignore_exists(self: &mut Self) -> FcResult<()> {
        self.create_ignore_exists()?;
        Ok(())
    }

    /// Returns a string containing the Debug representation of the path of
    /// the file corresponding to the specified name. If there's an error
    /// retrieving the path, it will contain the error instead.
    fn get_debug_info_for_file<NameRef: AsRef<OsStr>>(&self, name: NameRef) -> String {
        format!("path: {:#?}", self.get_file_path(name))
    }
}