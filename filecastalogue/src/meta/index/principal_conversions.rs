use std::io::Read;
use std::convert::{TryFrom, TryInto};
use std::collections::HashMap;
use std::ffi::OsString;
use crate::{error::{Error, ErrorKind, WrappedError}, meta::blob::model::Blob};
use super::model::{Index, UnicodePathIndex};

/// Principal conversions between Index and various other forms.
/// 
/// By convention, only these conversions should be used in order
/// to obtain the associated forms, in order to prevent code sprawl
/// of the processes used for these conversions. That way, there's
/// only one way to, say, obtain a Blob from an Index, which helps
/// with the maintainability of code that depends on that, such as
/// hashing related code.

impl TryFrom<Index> for UnicodePathIndex {
    type Error = Error;

    /// Principal Conversion from Index to UnicodePathIndex.
    /// 
    /// This is the one way which should be used to obtain a
    /// UnicodePathIndex from an Index.
    /// 
    /// At the moment, this converts OsStrings to Strings by
    /// the means of `.to_string_lossy`.
    fn try_from(index: Index) -> Result<Self, Self::Error> {
        let mut unicode_path_index = UnicodePathIndex {
            files: HashMap::new()
        };
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
    }
}

impl TryFrom<UnicodePathIndex> for Index {
    type Error = Error;

    fn try_from(unicode_path_index: UnicodePathIndex) -> Result<Self, Self::Error> {
        let mut index = Index::new();
        for (k_path, v_aspects) in unicode_path_index.files {
            // TODO: Switch between non-unicode-support and unicode-only
            //  feature (maybe).
            index.files.insert(serde_json::from_str(&k_path)?, v_aspects);
        }
        Ok(index)
    }
}

impl TryFrom<&mut (dyn Read)> for Index {
    type Error = Error;

    /// Principal conversion from Read to Index.
    /// 
    /// This is the one way which should be used to obtain a
    /// deserialized Index directly from a Read providing a
    /// Blob. This wraps around the principal conversion of
    /// Blob to Index.
    /// 
    /// The Read must produce a serde_json deserializable Blob
    /// or this will fail.
    fn try_from(readable: &mut (dyn Read)) -> Result<Self, Self::Error> {
        let blob: Blob = readable.try_into()?;
        let unicode_path_index: UnicodePathIndex = blob.try_into()?;
        Ok(unicode_path_index.try_into()?)
    }
}

impl TryFrom<Blob> for UnicodePathIndex {
    type Error = Error;

    /// Principal conversion from Blob to Index.
    /// 
    /// This is the one way which should be used to obtain a deserialized
    /// UnicodePathIndex from a Blob.
    /// 
    /// The Blob must be serde_json deserializable or this will fail.
    fn try_from(blob: Blob) -> Result<Self, Self::Error> {
        Ok(serde_json::from_slice(&blob)?)
    }
}

impl TryFrom<Index> for Blob {
    type Error = Error;

    /// Principal conversion from Index to Blob.
    /// 
    /// This is the one way which should be used to obtain a Blob from
    /// an Index.
    /// 
    /// This converts the paths of the Index to their unicode representation
    /// and uses `serde_json::to_vec_pretty` to create the blob.
    fn try_from(index: Index) -> Result<Self, Self::Error> {
        let unicode_path_index: UnicodePathIndex = index.try_into()?;
        match serde_json::to_vec_pretty(&unicode_path_index) {
            Ok(index) => Ok(index.into()),
            Err(e) => Err(error!(
                kind => ErrorKind::RepoFileOperationFailed,
                context => "Trying to convert Index to Blob.",
                // NOTE [security]: This contains file paths, which are
                //  potentially sensitive data.
                payload => format!("{:?}", unicode_path_index),
                wrapped => WrappedError::Serde(e)
            )),
        }
    }
}