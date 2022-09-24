use std::{collections::HashMap, ffi::{OsString, OsStr}};
use serde::{Deserialize, Serialize};

use super::super::file_aspects::enums::TrackedFileAspects;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Index {
    pub files: HashMap<OsString, TrackedFileAspects>
}

/// The serializable version of `Index`, with `String` keys instead
/// of `OsString`.
/// 
/// This deviates from the <path, aspects> top level map JSON model
/// of the Index insofar as the map is held in the `files` attribute,
/// which doesn't exist in the (flattened) serialized representation.
/// 
/// This struct is intended for (de)serialization and not any other uses.
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct UnicodePathIndex {
    // `files` is just an attribute-shaped representation of the top level
    // "dict" the JSON representation of the Index actually is, which is
    // why we're using `flatten` here. That way we don't get a `files`
    // attribute in JSON, but everything in `files` is popped right into
    // the JSON's top level instead.
    #[serde(flatten)]
    pub files: HashMap<String, TrackedFileAspects>
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
        // TODO [dummy-cleanup]: Proper implementation.
        String::new()
    }
}



        

//         // TODO: Write proper conversion to error::Error, so unwrap can be replaced
