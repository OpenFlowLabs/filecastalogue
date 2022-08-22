use std::{collections::HashMap, ffi::{OsString, OsStr}};
use serde::{Deserialize, Serializer, Serialize, ser::SerializeMap};
use super::super::file_aspects::enums::TrackedFileAspects;

#[derive(Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Index {
    pub files: HashMap<OsString, TrackedFileAspects>
}

impl Serialize for Index {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer {
        let mut serialized = serializer.serialize_map(Some(self.files.len()))?;
        
        // Naming philosophy:
        //   `k_` for key and `v_` for value as prefixes, followed
        //   by a descriptive name. This seems useful, since the
        //   struct we're implementing something for (`Index`)
        //   is non-generic in nature and is tightly wound around very
        //   specific domain items (a "path" key and a certain business
        //   logic related data structure).
        for (k_path, v_aspects) in &self.files {
            // TODO: Investigate `serialize_entry`'s behaviour for
            // non-unicode OsString keys. This is also a good point
            // to look into non-unicode related improvements and
            // error handling (at least as far as serialization is 
            // concerned). Right now, this comes with a high potential
            // for confusing malfunctions.
            serialized.serialize_entry(&k_path, &v_aspects)?;
        }
        serialized.end()
    }
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