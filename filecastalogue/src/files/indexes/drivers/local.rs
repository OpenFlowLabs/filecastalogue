use std::{ffi::OsStr, rc::Rc};
use crate::{
    error::FcResult, files::{
        RepoFile, index::{
            IndexProvider,
            drivers::local::{IndexFile, RepoIndexFile}},
            indexes::IndexFileCollection},
    finite_stream_handlers::LocalFile,
    opaque_collection_handlers::OpaqueCollectionHandler};

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
    fn has_index(self: &mut Self, index: &str) -> bool {
        todo!()
    }

    fn add_index(self: &mut Self)
    -> FcResult<&mut(dyn IndexFileCollection)> {
        Ok(self)
    }

    fn get_index_file<'ifile>(self: &mut Self, index: &str)
    -> FcResult<Rc<(dyn RepoIndexFile)>> {
        let handler: LocalFile = self.handler.get_file(OsStr::new(index))?;
        let index_file: Rc<(dyn RepoIndexFile)> = Rc::new(IndexFile::new(handler)?);
        Ok(index_file)
    }
}