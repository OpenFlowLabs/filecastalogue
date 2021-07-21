use crate::{error::{Error, ErrorKind, FcResult}};
use super::{
    error::{VersionEntryAlreadyExistsErrorPayload,
        VersionEntryDoesNotExistErrorPayload},
    model::{State, Version}};

pub trait Accessor<'acc> {
    fn has_version(self: &mut Self, id: &str) -> bool;
    fn get_version(self: &mut Self, id: &str)
    -> FcResult<Version>;
    // fn get_version_entry(self: &'acc mut Self, id: &'acc str)
    // -> Result<OccupiedEntry<'acc, String, Version>, VersionEntryDoesNotExistError<'acc>>;
    fn put_version<'f>(self: &mut Self, id: &'f str, index: &'f str) -> &mut Self;
    fn add_version(self: &mut Self, id: &str, index: &str)
    -> FcResult<&mut Self>;
    fn del_version(self: &mut Self, id: &str)
    -> FcResult<&mut Self>;
}

impl<'acc> Accessor<'acc> for State {
//     fn has_version(&mut self, id: &str)
//     -> Result<&mut Self, VersionEntryDoesNotExistError> {
//         if self.versions.contains_key(id) {
//             Ok(self)
//         }
//         else {
//             Err(VersionEntryDoesNotExistError {
//                 version_id: id.to_owned(),
//                 context_description: "Checking if there's an entry for that version.".to_owned()
//             })
//         }
//     }

    fn has_version(self: &mut Self, id: &str) -> bool {
        if self.versions.contains_key(id) {
            true
        }
        else {
            false
        }
    }

    // Experiment trying to have lifetime annotated entries.
    // Ran into other problems with entries related to mutable reference ownership and whatnot, though.
    // Leaving it here for the moment, as other experiments pertaining to hash-content access are ongoing.
    // fn get_version_entry(self: &'acc mut Self, id: &'acc str)
    // -> Result<OccupiedEntry<'acc, String, Version>, VersionEntryDoesNotExistError<'acc>> {
    //     let entry = self.versions.entry(id.to_owned());
    //     match entry {
    //         Entry::Occupied(version_entry)
    //         => Ok(version_entry),
    //         Entry::Vacant(version_entry)
    //         => Err(VersionEntryDoesNotExistError {
    //             version_id: id,
    //             context_description: "Adding a version entry."
    //         })
    //     }
    // }

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

    fn put_version<'f>(&mut self, id: &'f str, index: &'f str) -> &mut Self {
        self.versions.insert(id.to_owned(),Version {
            index: index.to_owned()
        });
        self
    }

    fn add_version(self: &mut Self, id: &str, index: &str)
    -> FcResult<&mut Self> {
        match self.get_version(id) {
            Ok(version) => Err(error!(
                ErrorKind::VersionEntryAlreadyExists,
                "Adding a version entry.",
                payload => VersionEntryAlreadyExistsErrorPayload {
                    version_id: id.to_owned(),
                    version_struct: version.to_owned(),
                }
            )),
            Err(_) => Ok(self.put_version(id, index))
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