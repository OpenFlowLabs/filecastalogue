use std::{collections::HashMap, ffi::OsString};
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
}