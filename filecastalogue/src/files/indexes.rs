use core::fmt;
use std::{error::Error, rc::Rc};
use crate::error::FcResult;
use super::index::drivers::local::RepoIndexFile;

pub mod drivers {
    pub mod local;
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
    IoError(AddingNewIndexFailedError<std::io::Error>),
    // Test whether this breaks things.
    Test(String)
}

pub trait IndexFileCollection {
    fn has_index(self: &mut Self, index: &str) -> bool;
    fn add_index(self: &mut Self)
    -> FcResult<&mut(dyn IndexFileCollection)>;
    fn get_index_file(self: &mut Self, index: &str)
    -> FcResult<Rc<(dyn RepoIndexFile)>>;
}