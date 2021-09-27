use core::fmt;
use std::error::Error;
use crate::error::FcResult;
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
        self: &mut Self, index_file: &'putting mut (dyn RepoIndexFile))
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
    // This returning "IndexFileCollection" doesn't look quite right.. :p
    -> Box<(dyn RepoIndexFile)> {
        Box::new(IndexFile::new())
    }
    
    /// Get an index file from the collection.
    fn get_index_file(self: &mut Self, hash: &str)
    -> FcResult<Box<(dyn RepoIndexFile)>> {
        let mut reader = self.handler.get_file_readable(
            OsStr::new(hash)
        )?;
        let index_file: Box<(dyn RepoIndexFile)> = Box::new(
            IndexFile::from_existing(
                &mut reader
            )?
        );
        Ok(index_file)
    }

    /// Save an index file to the collection.
    /// This will get the hash of the file's contents, write them to
    /// to a file with the hash for a name and return the hash.
    fn put_index_file(
        self: &mut Self, index_file: &mut (dyn RepoIndexFile))
    -> FcResult<String> {
        let hash = index_file.get_hash()?;
        let mut writeable = self.handler.get_file_writeable(
            OsStr::new(&hash))?;
        index_file.save(&mut writeable)?;
        Ok(hash)
    }
}

pub struct AddingNewIndexFailedError<WrappedError: Error + 'static> {
    pub wrapped_error: WrappedError,
    pub context_description: String,
}

impl<WrappedError: Error> AddingNewIndexFailedError<WrappedError> {
    pub fn new(context_description: &str, error: WrappedError)
    -> Self {
        Self {
            context_description: context_description.to_owned(),
            wrapped_error: error
        }
    }
}

impl<WrappedError: Error> fmt::Debug for AddingNewIndexFailedError<WrappedError> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Error: Adding a new index failed: \"{:?}\". Context: {}",
            self.wrapped_error,
            self.context_description
        )
    }
}

impl<WrappedError: Error> fmt::Display for AddingNewIndexFailedError<WrappedError> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Error: Adding a new index failed: \"{}\".",
            self.wrapped_error
        )
    }
}

impl<WrappedError: Error> Error for AddingNewIndexFailedError<WrappedError> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.wrapped_error)
    }
}

pub enum AddingNewIndexFailedErrors {
    IoError(AddingNewIndexFailedError<std::io::Error>)
}