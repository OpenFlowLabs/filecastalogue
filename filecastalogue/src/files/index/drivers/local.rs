use std::io;

use crate::{error::FcResult, files::RepoFile, finite_stream_handlers::FiniteStreamHandler, meta::index::model::Index};

pub struct IndexFile<Handler> where Handler: FiniteStreamHandler {
    pub handler: Handler,
    pub index: Index,
}

impl<Handler: FiniteStreamHandler> IndexFile<Handler> {
    pub fn new(handler: Handler) -> Result<Self, io::Error> {
        let mut mut_handler = handler;
        Ok(Self {
            index: mut_handler.read_all()?,
            handler: mut_handler,
        })
    }
}

impl<Handler: FiniteStreamHandler> RepoFile for IndexFile<Handler> {
    fn load(self: &mut Self) -> FcResult<&mut Self> {
        todo!()
    }

    fn save(self: &mut Self) -> FcResult<&mut Self> {
        todo!()
    }
}