use crate::error::{Error, ErrorKind, FcResult, KeyValuePayload, WrappedError};
use super::{index::{IndexFile, RepoIndexFile}};
use std::ffi::OsStr;
use crate::opaque_collection_handlers::OpaqueCollectionHandler;

pub trait IndexFileCollection {
    fn has_index(self: &mut Self, index: &str) -> FcResult<bool>;
    fn create_unwritten_empty_index_file_box(&self)
    -> Box<(dyn RepoIndexFile)>;
    fn get_index_file(self: &mut Self, index: &str)
    -> FcResult<Box<(dyn RepoIndexFile)>>;
    fn put_index_file<'putting>(
        self: &mut Self, index_file: Box<dyn RepoIndexFile>)
    -> FcResult<String>;
}

// TODO: Evaluate the nature of this struct, as "its "local"
// nature has generalized a lot to "not really local" over
// the course of the refactoring to the Handler-based approach.
// Of course, there's also the hard coded dependency on
// LocalFile right now, which we might be able to dynamic
// dispatch away somehow.
// Right now, it's more of a "MiscIndexFileCollection",
// considering a vague notion that, in the future, specific
// behaviour beyond the basic implementation of a Handler
// might still be a relevant concern that may find the driver
// concept accommodating.
pub struct MiscIndexFileCollection<Handler> where Handler: OpaqueCollectionHandler {
    pub handler: Handler
}

impl<Handler: OpaqueCollectionHandler> MiscIndexFileCollection<Handler> {
    pub fn new(handler: Handler) -> Self {
        Self {
            handler: handler
        }
    }
}

impl<
    Handler: OpaqueCollectionHandler
> IndexFileCollection for MiscIndexFileCollection<Handler> {
    fn has_index(self: &mut Self, index: &str) -> FcResult<bool> {
        self.handler.has_file(index)
    }

    fn create_unwritten_empty_index_file_box(&self)
    -> Box<(dyn RepoIndexFile)> {
        Box::new(IndexFile::new())
    }
    
    /// Get an index file from the collection.
    fn get_index_file(self: &mut Self, hash: &str)
    -> FcResult<Box<(dyn RepoIndexFile)>> {
        let mut reader = self.handler.get_file_readable(
            OsStr::new(hash)
        )?;
        match IndexFile::from_existing(&mut reader) {
            Ok(index_file) => Ok(Box::new(index_file)),
            Err(e) => Err(Error::new(
                ErrorKind::RepoFileOperationFailed,
                "Reading and deserializing the contents of\
                an index file from an index file collection.",
                Some(Box::new(self.handler.get_debug_info_for_file(hash))),
                Some(WrappedError::Fc(Box::new(e)))
            )),
        }
    }

    /// Save an index file to the collection.
    /// This will get the hash of the file's contents, write them to
    /// to a file with the hash for a name and return the hash.
    fn put_index_file(
        self: &mut Self, index_file: Box<dyn RepoIndexFile>)
    -> FcResult<String> {
        let hash = index_file.get_hash()?;
        if !self.handler.has_file(&hash)? {
            self.handler.create_file(&hash)?
        };
        let mut writeable = self.handler.get_file_writeable(
            OsStr::new(&hash))?;
        // TODO: This doesn't look right. ^^"
        let mut index_file = index_file;
        match index_file.save(&mut writeable) {
            Ok(_) => Ok(hash),
            Err(error) => Err(error!(
                ErrorKind::PuttingFileIntoCollectionFailed,
                "Putting index file into collection.",
                payload!("File name: ", Box::new(hash.clone())),
                WrappedError::Fc(Box::new(error))
            ))
        }
    }
}
