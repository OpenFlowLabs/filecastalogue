use crate::{error::{Error, ErrorKind, FcResult}, meta::version::model::Version};
use super::{
    error::{VersionEntryAlreadyExistsErrorPayload,
        VersionEntryDoesNotExistErrorPayload},
    model::State};

pub trait Accessor<'acc> {
    fn has_version(self: &mut Self, id: &str) -> bool;
    fn get_version(self: &mut Self, id: &str)
    -> FcResult<Version>;
    // fn get_version_entry(self: &'acc mut Self, id: &'acc str)
    // -> Result<OccupiedEntry<'acc, String, Version>, VersionEntryDoesNotExistError<'acc>>;
    fn put_version<'f>(self: &mut Self, id: &'f str, version: Version) -> &mut Self;
    fn add_version(self: &mut Self, id: &str, version: Version)
    -> FcResult<&mut Self>;
    fn del_version(self: &mut Self, id: &str)
    -> FcResult<&mut Self>;
}

impl<'acc> Accessor<'acc> for State {

    fn has_version(self: &mut Self, id: &str) -> bool {
        if self.versions.contains_key(id) {
            true
        }
        else {
            false
        }
    }

    fn get_version(self: &mut Self, id: &str)
    -> FcResult<Version> {
        match self.versions.get(id) {
            Some(version) => Ok(version.to_owned()),
            None => Err(error!(
                ErrorKind::VersionEntryDoesNotExist,
                "Getting entry for that version.",
                payload => VersionEntryDoesNotExistErrorPayload {
                    version_id: id.to_owned(),
                }
            ))
        }
    }

    // Version is consumed here, in order to force explicit handling of
    // situations where the version has to continue being available in the
    // calling context.
    fn put_version<'f>(&mut self, id: &'f str, version: Version) -> &mut Self {
        self.versions.insert(id.to_owned(),version);
        self
    }

    // Version is consumed here, in order to force explicit handling of
    // situations where the version has to continue being available in the
    // calling context.
    fn add_version(self: &mut Self, id: &str, version: Version)
    -> FcResult<&mut Self> {
        match self.get_version(id) {
            Ok(version) => Err(error!(
                ErrorKind::VersionEntryAlreadyExists,
                "Adding a version entry.",
                payload => VersionEntryAlreadyExistsErrorPayload {
                    version_id: id.to_owned(),
                    version_struct: version,
                }
            )),
            Err(_) => Ok(self.put_version(id, version))
        }
    }

    fn del_version(self: &mut Self, id: &str)
    -> FcResult<&mut Self> {
        match self.versions.remove_entry(id) {
            Some(_) => Ok(self),
            None => Err(error!(
                ErrorKind::VersionEntryDoesNotExist,
                "Deleting a version entry.",
                payload=> VersionEntryDoesNotExistErrorPayload { 
                    version_id: id.to_owned(),
                }
            ))
        }
    }
}