use std::fmt;
//use std::collections::hash_map::{Entry, OccupiedEntry};
use crate::meta::state::model::{State, Version};


pub struct VersionEntryAlreadyExistsError {
    pub version_id: String,
    pub version_struct: Version,
    pub context_description: String,
}

impl<'err> fmt::Debug for VersionEntryAlreadyExistsError {
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

impl<'err> fmt::Display for VersionEntryAlreadyExistsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Error: An entry for version \"{}\" already exists.",
            self.version_id,
        )
    }
}

pub struct VersionEntryDoesNotExistError {
    pub version_id: String,
    pub context_description: String,
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
            "Error: There's no entry for version \"{}\".",
            self.version_id
        )
    }
}

pub trait Accessor<'acc> {
    fn has_version(self: &mut Self, id: &str) -> bool;
    fn get_version(self: &mut Self, id: &str)
    -> Result<Version, VersionEntryDoesNotExistError>;
    // fn get_version_entry(self: &'acc mut Self, id: &'acc str)
    // -> Result<OccupiedEntry<'acc, String, Version>, VersionEntryDoesNotExistError<'acc>>;
    fn put_version<'f>(self: &mut Self, id: &'f str, index: &'f str) -> &mut Self;
    fn add_version(self: &mut Self, id: &str, index: &str)
    -> Result<&mut Self, VersionEntryAlreadyExistsError>;
    fn del_version(self: &mut Self, id: &str)
    -> Result<&mut Self, VersionEntryDoesNotExistError>;
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
    -> Result<Version, VersionEntryDoesNotExistError> {
        match self.versions.get(id) {
            Some(version) => Ok(version.to_owned()),
            None => Err(VersionEntryDoesNotExistError {
                version_id: id.to_owned(),
                context_description: String::from("Getting entry for that version.")
            })
        }
    }

    fn put_version<'f>(&mut self, id: &'f str, index: &'f str) -> &mut Self {
        self.versions.insert(id.to_owned(),Version {
            index: index.to_owned()
        });
        self
    }

    fn add_version(self: &mut Self, id: &str, index: &str)
    -> Result<&mut Self, VersionEntryAlreadyExistsError> {
        match self.get_version(id) {
            Ok(version) => Err(VersionEntryAlreadyExistsError {
                version_id: id.to_owned(),
                version_struct: version.to_owned(),
                context_description: String::from("Adding a version entry.")
            }),
            Err(_) => Ok(self.put_version(id, index))
        }
    }

    fn del_version(self: &mut Self, id: &str)
    -> Result<&mut Self, VersionEntryDoesNotExistError> {
        match self.versions.remove_entry(id) {
            Some(_) => Ok(self),
            None => Err(VersionEntryDoesNotExistError { 
                version_id: id.to_owned(),
                context_description: String::from("Deleting a version entry.")
            })
        }
    }
}