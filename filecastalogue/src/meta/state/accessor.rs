use std::fmt;
use crate::meta::state::model::{State, Version};

struct VersionEntryAlreadyExistsError {
    context_description: String,
    version_id: String,
    version_struct: Version
}

impl fmt::Debug for VersionEntryAlreadyExistsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Error: An entry for version \"{}\" already exists and contains: {:?}. Context: {}",
            self.version_id,
            self.version_struct,
            self.context_description
        )
    }
}

impl fmt::Display for VersionEntryAlreadyExistsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Error: An entry for version \"{}\" already exists. Context: {}",
            self.version_id,
            self.context_description,
        )
    }
}

struct VersionEntryDoesNotExistError {
    context_description: String,
    version_id: String,
}

impl fmt::Debug for VersionEntryDoesNotExistError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Error: There's no entry for version \"{}\". Context: {}",
            self.version_id,
            self.context_description
        )
    }
}

impl fmt::Display for VersionEntryDoesNotExistError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Error: There's no entry for version \"{}\". Context: {}",
            self.version_id,
            self.context_description,
        )
    }
}

pub enum Error {
    VersionEntryAlreadyExistsError
}
pub trait Accessor {
    fn has_version(self: &mut Self, id: String)
    -> Result<&mut Self, VersionEntryDoesNotExistError>;
    fn get_version(self: &mut Self, id: String) -> Result<Version, VersionEntryDoesNotExistError>;
    fn put_version(self: &mut Self, id: String, index: String) -> &mut Self;
    fn add_version(self: &mut Self, id: String, index: String)
    -> Result<&mut Self, VersionEntryAlreadyExistsError>;
    fn del_version(self: &mut Self, id: String) -> &mut Self;
}

impl Accessor for State {
    fn has_version(&mut self, id: String)
    -> Result<&mut Self, VersionEntryDoesNotExistError> {
        if self.versions.contains_key(&id) {
            Ok(self)
        }
        else {
            Err(VersionEntryDoesNotExistError {
                version_id: id,
                context_description: String::from(
                    "Checking if there's an entry for that version."
                )
            })
        }
    }

    fn get_version(self: &mut Self, id: String)
    -> Result<Version, VersionEntryDoesNotExistError> {
        match self.versions.get(id) {
            Some(version) => Ok(version),
            None => Err(VersionEntryDoesNotExistError {
                version_id: id,
                context_description: String::from("Getting entry for that version.")
            })
        }
    }

    fn put_version(&mut self, id: String, index: String) -> &mut Self {
        self.versions.insert(id,Version {
            index: String::from("TODO")
        });
        self
    }

    fn add_version(self: &mut Self, id: String, index: String)
    -> Result<&mut Self, VersionEntryAlreadyExistsError> {
        match self.has_version(id) {
            Ok(_) => Err(VersionEntryAlreadyExistsError {
                version_id: id,
                // TODO: Actually implement.
                version_struct: Version {index: String::from("TODO")},
                context_description: String::from(
                    "Adding a version entry."
                )
            }),
            Err(_) => Ok(self)
        }
    }
}