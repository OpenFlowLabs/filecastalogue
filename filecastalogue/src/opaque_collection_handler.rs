use std::ffi::OsStr;
use std::fmt::Display;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use crate::error::FcResult;
use crate::error::ErrorPathBuf;
use crate::error::Payload;

pub mod drivers {
    pub mod local;
}

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