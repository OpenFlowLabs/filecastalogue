use crate::{files::{indexes::IndexFileCollection},
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

impl<Handler: OpaqueCollectionHandler> IndexFileCollection for LocalIndexFileCollection<Handler> {
    fn has_index(self: &mut Self, index: &str) -> bool {
        todo!()
    }

    fn add_index(self: &mut Self)
    -> Result<&mut Self, crate::files::indexes::AddingNewIndexFailedErrors> {
        todo!()
    }
}