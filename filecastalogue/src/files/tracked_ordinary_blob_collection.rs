use std::ffi::{OsStr, OsString};
use crate::{error::FcResult,
    opaque_collection_handlers::OpaqueCollectionHandler};
use super::tracked_ordinary_blob::{RepoTrackedOrdinaryBlobFile, TrackedOrdinaryBlobFile};

pub trait TrackedOrdinaryBlobFileCollection {
    fn has_file(self: &mut Self, hash: &str) -> FcResult<bool>;
    fn get_file(self: &mut Self, hash: &str)
    -> FcResult<Box<dyn RepoTrackedOrdinaryBlobFile>>;
    fn put_file(
        self: &mut Self, tracked_file: &mut (dyn RepoTrackedOrdinaryBlobFile))
    -> FcResult<String>;
}

pub struct MiscTrackedOrdinaryBlobFileCollection<Handler>
where Handler: OpaqueCollectionHandler<> {
    pub handler: Handler
}

impl<Handler: OpaqueCollectionHandler> MiscTrackedOrdinaryBlobFileCollection<Handler> {
    pub fn new(handler: Handler) -> Self {
        Self {
            handler: handler
        }
    }
}

impl<Handler: OpaqueCollectionHandler> TrackedOrdinaryBlobFileCollection
for MiscTrackedOrdinaryBlobFileCollection<Handler> {
    fn has_file(self: &mut Self, hash: &str) -> FcResult<bool> {
        self.handler.has_file(hash)
    }

    fn get_file(self: &mut Self, hash: &str)
    -> FcResult<Box<dyn RepoTrackedOrdinaryBlobFile>> {
        let mut readable = self.handler.get_file_readable(
            OsStr::new(hash)
        )?;
        let tracked_file: Box<dyn RepoTrackedOrdinaryBlobFile> = Box::new(
            TrackedOrdinaryBlobFile::from_existing(
                &mut readable
            )?
        );
        Ok(tracked_file)
    }

    fn put_file(
        self: &mut Self, tracked_file: &mut (dyn RepoTrackedOrdinaryBlobFile))
    -> FcResult<String> {
        let hash = tracked_file.get_hash()?;
        let mut writeable = self.handler.get_file_writeable(
            &OsString::from(&hash))?;
        tracked_file.save(&mut writeable)?;
        Ok(hash)
    }
}