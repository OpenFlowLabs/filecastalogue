use std::{rc::Rc};
use crate::{error::FcResult, files::{
        index::{
            drivers::local::{IndexFile, RepoIndexFile}},
            indexes::IndexFileCollection},
            opaque_collection_handlers::OpaqueCollectionHandler};

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
pub struct LocalIndexFileCollection<Handler> where Handler: OpaqueCollectionHandler {
    pub handler: Handler
}

impl<Handler: OpaqueCollectionHandler> LocalIndexFileCollection<Handler> {
    pub fn new(handler: Handler) -> Self {
        Self {
            handler: handler
        }
    }
}

// trait Test: RepoFile + IndexProvider {}

impl<
    Handler: OpaqueCollectionHandler
> IndexFileCollection for LocalIndexFileCollection<Handler> {
    fn has_index(self: &mut Self, index: &str) -> FcResult<bool> {
        self.handler.has_file(index)
    }

    fn get_new_index_file(self: &mut Self, index_file: &(dyn RepoIndexFile))
    // This returning "IndexFileCollection" doesn't look quite right.. :p
    -> FcResult<&mut(dyn IndexFileCollection)> {
        todo!(); // get_new_file will need a proper name specified, not "".
        // let index_file: = IndexFile::new(self.handler.get_new_file("")?);
    }

    fn get_index_file<'ifile>(self: &mut Self, index: &str)
    -> FcResult<Rc<(dyn RepoIndexFile)>> {
        let mut reader = self.handler.get_file_reader(index)?;
        let index_file: Rc<(dyn RepoIndexFile)> = Rc::new(
            IndexFile::from_existing(
                &mut reader
            )?
        );
        Ok(index_file)
    }
}