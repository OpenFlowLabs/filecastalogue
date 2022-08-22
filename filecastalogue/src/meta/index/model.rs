use std::{collections::HashMap, ffi::{OsString, OsStr}};
use serde::{Serialize, Deserialize};
use super::super::file_aspects::enums::TrackedFileAspects;

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Index {
    pub files: HashMap<OsString, TrackedFileAspects>
}

impl Index {
    pub fn new() -> Self {
        Self {
            files: HashMap::new()
        }
    }

    #[cfg(not(feature = "os_string_path"))]
    pub fn string_from_os_string(os_string: &OsStr) -> String {
        // Just testing. Using a lossy conversion would
        // defeat the entire purpose of using OsString paths.
        os_string.to_string_lossy().to_string()
    }

    #[cfg(feature = "os_string_path")]
    pub fn string_from_os_string(os_string: &OsStr) -> String {
        // TODO ]dummy-cleanup]: Proper implementation.
        String::new()
    }

}