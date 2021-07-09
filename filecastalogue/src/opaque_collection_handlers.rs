use std::{ffi::{OsStr, OsString}, fmt::Display, fs::{File, create_dir},
io::{Read, Write}, path::{Path, PathBuf}, rc::Rc};
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

    fn get_file<NameRef: AsRef<OsStr>>(&self, name: NameRef) -> FcResult<File> {
        let path = self.get_file_path(name)?;
        match path.exists() {
            true => Ok(File::open(path.to_owned())?),
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
    fn get_file_reader(&self, name: &OsStr)
    -> FcResult<Box<(dyn Read)>>;
    fn get_file_writer(&self, name: &OsStr)
    -> FcResult<Rc<(dyn Write)>>;
    fn collection_exists(self: &mut Self) -> bool;
    fn create_collection(self: &mut Self) -> FcResult<()>;
    fn create_collection_ignore_exists(self: &mut Self) -> FcResult<()>;
}

impl OpaqueCollectionHandler for LocalDir
{
    fn has_file<NameRef: AsRef<OsStr>>(self: &mut Self, name: NameRef)
    -> FcResult<bool> {
        Ok(self.get_file_path(name)?.exists())
    }

    fn get_file_reader(&self, name: &OsStr)
    -> FcResult<Box<(dyn Read)>> {
        Ok(Box::new(self.get_file(name)?))
    }

    fn get_file_writer(&self, name: &OsStr)
    -> FcResult<Rc<(dyn Write)>> {
        Ok(Rc::new(self.get_file(name)?))
    }

    fn collection_exists(self: &mut Self) -> bool {
        self.path.exists()
    }

    fn create_collection(self: &mut Self) -> FcResult<()> {
        self.create()?;
        Ok(())
    }

    fn create_collection_ignore_exists(self: &mut Self) -> FcResult<()> {
        self.create_ignore_exists()?;
        Ok(())
    }
}