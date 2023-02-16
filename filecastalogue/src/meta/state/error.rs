use std::fmt;
use crate::error::Payload;
use super::super::version::model::Version;

pub struct VersionEntryAlreadyExistsErrorPayload {
    pub version_index: usize,
    pub version_struct: Version,
}

impl<'err> fmt::Debug for VersionEntryAlreadyExistsErrorPayload {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "An entry for version \"{}\" already exists and contains: {:?}.",
            self.version_index,
            self.version_struct,
        )
    }
}

impl<'err> fmt::Display for VersionEntryAlreadyExistsErrorPayload {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "An entry for version \"{}\" already exists.",
            self.version_index,
        )
    }
}

impl Payload for VersionEntryAlreadyExistsErrorPayload {}

pub struct VersionEntryDoesNotExistErrorPayload {
    pub version_index: usize,
}

impl fmt::Debug for VersionEntryDoesNotExistErrorPayload {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Error: There's no entry for version \"{}\".",
            self.version_index,
        )
    }
}

impl fmt::Display for VersionEntryDoesNotExistErrorPayload {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Error: There's no entry for version \"{}\".",
            self.version_index
        )
    }
}

impl Payload for VersionEntryDoesNotExistErrorPayload {}