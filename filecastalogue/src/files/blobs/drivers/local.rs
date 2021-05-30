use crate::{files::{blobs::BlobFileCollection},
opaque_collection_handlers::OpaqueCollectionHandler};

pub struct LocalBlobFileCollection<Handler> where Handler: OpaqueCollectionHandler {
    pub handler: Handler
}

impl<Handler: OpaqueCollectionHandler> LocalBlobFileCollection<Handler> {
    pub fn new(handler: Handler) -> Self {
        Self {
            handler: handler
        }
    }
}

impl<Handler: OpaqueCollectionHandler> BlobFileCollection for LocalBlobFileCollection<Handler> {}