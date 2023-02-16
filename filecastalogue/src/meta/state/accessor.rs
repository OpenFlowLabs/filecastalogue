use crate::{error::{Error, ErrorKind, FcResult}, meta::version::model::Version};
use super::{
    error::{VersionEntryAlreadyExistsErrorPayload,
        VersionEntryDoesNotExistErrorPayload},
    model::State};

pub trait StateAccessor<'acc> {
    fn has_version(self: &mut Self, version_index: usize) -> bool;
    fn get_version(self: &mut Self, version_index: usize)
    -> FcResult<Version>;
    // fn get_version_entry(self: &'acc mut Self, id: &'acc str)
    // -> Result<OccupiedEntry<'acc, String, Version>, VersionEntryDoesNotExistError<'acc>>;
    fn put_version<'f>(self: &mut Self, version_index: &'f usize, version: Version) -> &mut Self;
    fn add_version(self: &mut Self, version: Version)
    -> usize;
    fn del_version(self: &mut Self, version_index: usize)
    -> FcResult<&mut Self>;
}

impl<'acc> StateAccessor<'acc> for State {

    fn has_version(self: &mut Self, version_index: usize) -> bool {
        self.versions.len() > version_index
    }

    fn get_version(self: &mut Self, version_index: usize)
    -> FcResult<Version> {
        match self.versions.get(version_index) {
            Some(version) => Ok(version.to_owned()),
            None => Err(error!(
                ErrorKind::VersionEntryDoesNotExist,
                "Getting entry for that version.",
                payload => VersionEntryDoesNotExistErrorPayload {
                    version_index: version_index.to_owned(),
                }
            ))
        }
    }

    // Version is consumed here, in order to force explicit handling of
    // situations where the version has to continue being available in the
    // calling context.
    fn put_version<'f>(&mut self, version_index: &'f usize, version: Version) -> &mut Self {
        self.versions.insert(version_index.to_owned(),version);
        self
    }

    // Version is consumed here, in order to force explicit handling of
    // situations where the version has to continue being available in the
    // calling context.
    fn add_version(self: &mut Self, version: Version)
    -> usize {
        self.versions.push(version);
        self.versions.len() - 1
    }


    fn del_version(self: &mut Self, version_index: usize)
    -> FcResult<&mut Self> {
        if self.has_version(version_index) {
            self.versions.remove(version_index);
            Ok(self)
        }
        else {
            Err(error!(
                ErrorKind::VersionEntryDoesNotExist,
                "Deleting a version entry.",
                payload=> VersionEntryDoesNotExistErrorPayload { 
                    version_index: version_index.to_owned(),
                }
            ))
        }
    }

}