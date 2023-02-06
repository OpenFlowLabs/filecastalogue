use std::ffi::{OsStr, OsString};
use crate::error::FcResult;
use crate::opaque_collection_handler::OpaqueCollectionHandler;
use super::state::{RepoStateFile, StateFile};

pub trait StateFileCollection {
    fn has_state(self: &mut Self) -> FcResult<bool>;
    fn create_unwritten_empty_state_file_box(&self)
    -> Box<(dyn RepoStateFile)>;
    fn get_state_file(self: &mut Self)
    -> FcResult<Box<(dyn RepoStateFile)>>;
    fn put_state_file<'putting>(
        self: &mut Self, state_file: Box<dyn RepoStateFile>)
    -> FcResult<()>;
}

pub struct MiscStateFileCollection<Handler> where Handler: OpaqueCollectionHandler {
    pub handler: Handler,
    file_name: OsString
}

impl<Handler: OpaqueCollectionHandler> MiscStateFileCollection<Handler> {
    pub fn new(handler: Handler, file_name: OsString) -> Self {
        Self {
            handler: handler,
            file_name: file_name
        }
    }

    pub fn clone_file_name(&self) -> OsString {
        self.file_name.clone()
    }
}

impl<
    Handler: OpaqueCollectionHandler
> StateFileCollection for MiscStateFileCollection<Handler> {
    fn has_state(self: &mut Self) -> FcResult<bool> {
        let file_name = self.clone_file_name();
        self.handler.has_file(file_name)
    }

    fn create_unwritten_empty_state_file_box(&self)
    -> Box<(dyn RepoStateFile)> {
        Box::new(StateFile::new())
    }

    fn get_state_file(self: &mut Self)
    -> FcResult<Box<(dyn RepoStateFile)>> {
        let mut reader = self.handler.get_file_readable(
            &self.clone_file_name()
        )?;
        let state_file: Box<(dyn RepoStateFile)> = Box::new(
            StateFile::from_existing(
                &mut reader
            )?
        );
        Ok(state_file)

    }

    fn put_state_file<'putting>(
        self: &mut Self, state_file: Box<dyn RepoStateFile>)
    -> FcResult<()> {
        let file_name = self.clone_file_name();
        if !self.handler.has_file(&file_name)? {
            self.handler.create_file(&file_name)?
        };
        let mut writeable = self.handler.get_file_writeable(
            OsStr::new(&file_name)
        )?;
        let mut state_file = state_file;
        state_file.save(&mut writeable)?;
        Ok(())
    }
}