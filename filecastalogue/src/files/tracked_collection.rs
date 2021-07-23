use crate::opaque_collection_handlers::OpaqueCollectionHandler;
pub trait TrackedFileCollection {}

pub struct MiscTrackedFileCollection<Handler>
where Handler: OpaqueCollectionHandler<> {
    pub handler: Handler
}

impl<Handler: OpaqueCollectionHandler> MiscTrackedFileCollection<Handler> {
    pub fn new(handler: Handler) -> Self {
        Self {
            handler: handler
        }
    }
}

impl<Handler: OpaqueCollectionHandler> TrackedFileCollection for MiscTrackedFileCollection<Handler> {}