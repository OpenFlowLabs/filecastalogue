use std::{ffi::{OsStr, OsString}, fmt::Display, fs::create_dir, path::{Path, PathBuf}};
use crate::{error::{Error, ErrorKind, FcResult, KeyValuePayload, Payload}, finite_stream_handlers::FiniteStreamHandler};

#[derive(Debug)]
pub enum Problem {
    IndexFileAlreadyExists
}

impl Problem {
    pub(crate) fn as_str(&self) -> &'static str {
        match self {
            Self::IndexFileAlreadyExists => "Index file already exists."
        }
    }
}

impl Display for Problem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Debug)]
pub struct OpaqueCollectionErrorPayload {
    pub problem: Problem
}

impl OpaqueCollectionErrorPayload {
    fn new(problem: Problem) -> Self {
        Self {
            problem: problem
        }
    }
}

impl Display for OpaqueCollectionErrorPayload {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.problem
        )
    }
}

impl Payload for OpaqueCollectionErrorPayload {}

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
                payload => KeyValuePayload::new()
                .add("Original path", Box::new(file_name_path.to_owned()))
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

    fn exists(self: &mut Self) -> bool {
        self.path.exists()
    }

    /** Attempt to create the directory.
    */
    fn create(self: &mut Self) -> FcResult<&mut Self> {
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
    fn has_file(self: &mut Self, name: &OsStr) -> FcResult<bool>;
    fn get_file<T>(self: &mut Self, name: &OsStr)
    -> FcResult<T> where T: FiniteStreamHandler;
    fn get_new_file<T>(self: &mut Self, name: &OsStr)
    -> FcResult<T> where T: FiniteStreamHandler;
    fn collection_exists(self: &mut Self) -> bool;
    fn create_collection(self: &mut Self) -> FcResult<()>;
    fn create_collection_ignore_exists(self: &mut Self) -> FcResult<()>;
}

impl OpaqueCollectionHandler for LocalDir
{
    fn has_file(self: &mut Self, name: &OsStr) -> FcResult<bool> {
        Ok(self.get_file_path(name)?.exists())
    }

    /// Get a file from the collection by name.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::opaque_file_collection;
    ///
    /// let handler = LocalDir<T>
    /// let file: T = handler.get_file("existingfile")
    ///```
    fn get_file<T>(self: &mut Self, name: &OsStr)
    -> FcResult<T> where T: FiniteStreamHandler {
        let path = self.get_file_path(name)?;
        match path.exists() {
            true => Ok(T::new(path)),
            false => Err(error!(
                ErrorKind::CollectionHandlerOperationFailed,
                "Getting a file from the collection."
            ))
        }
        
    }

    fn get_new_file<T>(self: &mut Self, name: &OsStr)
    -> FcResult<T> where T: FiniteStreamHandler {
        let path = self.get_file_path(name)?;
        Ok(T::new(path))
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

    // fn read_file<T, NameRef: AsRef<OsStr>>(self: &mut Self, name: NameRef)
    // -> FcResult<T> {
    //     // let file = self.get_file(name)?;
    //     let file_path = self.get_file_path(name)?;
    //     IndexFile::new(LocalFile::new(file_path))
    // }
}