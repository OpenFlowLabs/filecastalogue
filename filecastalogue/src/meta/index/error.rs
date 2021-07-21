use std::{ffi::OsString, fmt};
use crate::error::Payload;
use super::model::Index;

pub struct FileAlreadyTrackedErrorPayload {
    pub path: OsString,
    pub index_struct: Index,
}

impl fmt::Debug for FileAlreadyTrackedErrorPayload {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Error: The file at path \"{:?}\" {} {:?}.",
            self.path,
            "is already tracked, with the following entry:",
            self.index_struct,
        )
    }
}

impl fmt::Display for FileAlreadyTrackedErrorPayload {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Error: The file at path \"{:?}\" is already tracked.",
            self.path,
        )
    }
}

impl Payload for FileAlreadyTrackedErrorPayload {}


pub struct UntrackedFileErrorPayload {
    pub path: OsString,
}

impl fmt::Debug for UntrackedFileErrorPayload {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "There is no file tracked for path \"{:?}\".",
            self.path
        )
    }
}

impl fmt::Display for UntrackedFileErrorPayload {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "There is no file tracked for path \"{:?}\".",
            self.path
        )
    }
}

impl Payload for UntrackedFileErrorPayload {}