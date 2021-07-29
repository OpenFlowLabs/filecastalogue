use core::fmt;
use std::{error::Error, ffi::OsString, rc::Rc};
use crate::error::FcResult;
use super::{index::{IndexFile, RepoIndexFile}};
use std::ffi::OsStr;
use crate::{opaque_collection_handlers::OpaqueCollectionHandler};

// TODO: Evaluate the nature of this struct, as "its "local"
// nature has mgeneralized a lot to "not really local" over
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

    fn get_new_index_file(self: &mut Self, index_file: &(dyn RepoIndexFile))
    // This returning "IndexFileCollection" doesn't look quite right.. :p
    -> FcResult<&mut(dyn IndexFileCollection)> {
        todo!(); // get_new_file will need a proper name specified, not "".
        // let index_file: = IndexFile::new(self.handler.get_new_file("")?);
    }

    // TODO: Do we really want this to be Rc?
    //  It's probably going to be cumbersome regarding mutability, and we might
    //  not want to provide too much flexibility in terms of spraying file
    //  related references everywhere. At the same time, getting the file,
    //  loading, getting the index, setting the index and saving ought to be
    //  straight forward and painless.
    //  Also, it's too early to tell how fancy things might be getting in Repo
    //  related code, particularly with journal/staging/consistency related
    //  things in mind, so maybe we don't want to be too restricting (yet?)
    //  in order not to shackle the prototyping process too much.
    /// Get an index file from the collection.
    fn get_index_file(self: &mut Self, hash: &str)
    -> FcResult<Rc<(dyn RepoIndexFile)>> {
        let mut reader = self.handler.get_file_readable(
            OsStr::new(hash)
        )?;
        let index_file: Rc<(dyn RepoIndexFile)> = Rc::new(
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
            &OsString::from(&hash))?;
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

pub trait IndexFileCollection {
    fn has_index(self: &mut Self, index: &str) -> FcResult<bool>;
    fn get_new_index_file(self: &mut Self, index_file: &(dyn RepoIndexFile))
    -> FcResult<&mut(dyn IndexFileCollection)>;
    fn get_index_file(self: &mut Self, index: &str)
    -> FcResult<Rc<(dyn RepoIndexFile)>>;
    fn put_index_file<'putting>(
        self: &mut Self, index_file: &'putting mut (dyn RepoIndexFile))
    -> FcResult<String>;
}