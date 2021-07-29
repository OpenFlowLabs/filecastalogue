use std::ffi::{OsStr, OsString};
use crate::{error::FcResult,
    opaque_collection_handlers::OpaqueCollectionHandler};
use super::tracked::{RepoTrackedFile, TrackedFile};

pub trait TrackedFileCollection {
    fn has_file(self: &mut Self, hash: &str) -> FcResult<bool>;
    fn get_file(self: &mut Self, hash: &str)
    -> FcResult<Box<dyn RepoTrackedFile>>;
    fn put_file(
        self: &mut Self, tracked_file: &mut (dyn RepoTrackedFile))
    -> FcResult<String>;
}

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

impl<Handler: OpaqueCollectionHandler> TrackedFileCollection
for MiscTrackedFileCollection<Handler> {
    fn has_file(self: &mut Self, hash: &str) -> FcResult<bool> {
        self.handler.has_file(hash)
    }

    fn get_file(self: &mut Self, hash: &str)
    -> FcResult<Box<dyn RepoTrackedFile>> {
        let mut readable = self.handler.get_file_readable(
            OsStr::new(hash)
        )?;
        let tracked_file: Box<dyn RepoTrackedFile> = Box::new(
            TrackedFile::from_existing(
                &mut readable
            )?
        );
        Ok(tracked_file)
    }

    fn put_file(
        self: &mut Self, tracked_file: &mut (dyn RepoTrackedFile))
    -> FcResult<String> {
        let hash = tracked_file.get_hash()?;
        let mut writeable = self.handler.get_file_writeable(
            &OsString::from(&hash))?;
        tracked_file.save(&mut writeable)?;
        Ok(hash)
    }
}