use std::{collections::HashMap, ffi::{OsString, OsStr}};
use serde::{Deserialize, Serialize};
use crate::error::FcResult;
use super::super::file_aspects::enums::TrackedFileAspects;

pub enum Conversion {
    LossyGraphemes,
    NonLossyBytes,
    // NonLossyGraphemesWithEscapedBytes - maybe?
}

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

    pub fn from_unicode_path_index(
        unicode_path_index: UnicodePathIndex,
        conversion_type: Conversion
    ) -> FcResult<Self> {
        let mut index = Index::new();
        match conversion_type {
            Conversion::LossyGraphemes => {
                for (k_path, v_aspects) in unicode_path_index.files {
                    index.files.insert(OsString::from(k_path), v_aspects);
                }
                Ok(index)
            },
            Conversion::NonLossyBytes => {
                for (k_path, v_aspects) in unicode_path_index.files {
                    index.files.insert(serde_json::from_str(&k_path)?, v_aspects);
                }
                Ok(index)
            }
        }
    }
}

impl UnicodePathIndex {
    pub fn from_index(
        index: Index, 
        conversion_type: Conversion
    ) -> FcResult<Self> {
        let mut unicode_path_index = Self {
            files: HashMap::new()
        };
        match conversion_type {
            Conversion::LossyGraphemes => {
                for (k_path, v_aspects) in index.files {
                    unicode_path_index.files.insert(k_path.to_string_lossy().to_string(), v_aspects);
                };
                Ok(unicode_path_index)
            },
            Conversion::NonLossyBytes => {
                for (k_path, v_aspects) in index.files {
                    // NOTE [caveat]: When processing path input in some way that might
                    //  have been serialized on another platform, take into account
                    //  that what might intuitively seem like it would have to be the
                    //  same string might still differ in the world of OsString, even
                    //  if it's just the "Unix" and "Windows" prefixes in their
                    //  serialized counterparts, which would e.g. make comparisons
                    //  between them fail.
                    unicode_path_index.files.insert(serde_json::to_string(&k_path)?, v_aspects);
                };
                Ok(unicode_path_index)
            },
        }
    }
}
