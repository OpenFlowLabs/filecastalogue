use crate::meta::blob::model::Blob;
use crate::{error::FcResult, meta::index::model::Index};
use std::convert::TryInto;
use std::io::{Read, Write};
use crate::files::hashable::Hashable;
use super::{RepoFile, blob::BlobProvider};

/**
Somehow provides index data and a means to set the index data
it provides, at least for as long as it's alive. Unless specifically
pointed out, the methods on the provier aren't concerned with
with where the data is coming from, where it goes, whether it's
persistent or anything like that.

Other means will have to provide for that (e.g. another trait).
*/
pub trait IndexProvider {
    fn get_index_ref(self: &mut Self) -> FcResult<&mut crate::meta::index::model::Index>;
    fn set_index(self: &mut Self, index: Index)
    -> FcResult<()>;
}

pub trait RepoIndexFile: RepoFile + IndexProvider + BlobProvider + Hashable {}


/// Represents a blob file holding an index.
/// 
/// Index blob files hold their blob in memory as a deserialized Index
/// struct. When the blob is needed in a u8 centric format, it has
/// to be serialized first. This is based on the assumption that u8
/// is only relevant to indexes in terms of loading, saving and hashing.
pub struct IndexFile {
    /// This is where the index is "cached" when it's loaded from
    /// the file or set by other means, and where it will be read from
    /// when saving the index to the file.
    /// 
    /// Loading the file will write to this, saving the file
    /// will read this (and write it to the file). In general, this is
    /// supposed to provide an in-place "workspace" to modify
    /// the index without persisting every single immediate step.
    /// 
    /// By convention, when setting our .index member from anything else
    /// than an already existing Index value, only the principal conversion
    /// into/try_into methods implemented for the Index model should be used
    /// to obtain the index used to do so.
    /// 
    /// Likewise, when converting the index value to something else, such
    /// as a blob, only use these conversion methods.
    /// 
    /// Using only principal methods for conversions ensures a
    /// single source of process of how indexes are transformed between
    /// their serialized and deserialized state, which part of the code
    /// is responsible for it and where to look when things go wrong.
    /// 
    /// For an overview, have a look at principal_conversions.rs of
    /// the index meta module.
    index: Index,
}

impl IndexFile {

    /// Create an IndexFile struct from a blob provided by a Read.
    /// 
    /// The blob needs to be JSON deserializable by serde_json.
    pub fn from_existing(readable: &mut (dyn Read)) -> FcResult<Self> {
        Ok(Self {
            index: readable.try_into()?
        })
    }
}

/// The interface for getting data from and to persistent storage.
/// 
/// This is supposed to be used by whatever is handling persistent storage
/// for index files, feeding a Read/Write implementation to the herein
/// implemented methods to provide us with an implementation agnostic
/// interface to the persistent storage.
impl RepoFile for IndexFile {
    
    /* Notes:
        If we end up not using .load for IndexFile, we could consider
        reworking/splitting RepoFile (e.g. into a load and save version),
        particularly if we won't use it for the Tracked file struct
        either.
    */
    /// Fill IndexFile with JSON data from a Read.
    /// 
    /// The mental model is that the Read represents the persistent storage
    /// of an index in JSON form. The data received needs to be deserializable
    /// by serde_json.
    fn load(self: &mut Self, readable: &mut (dyn Read)) -> FcResult<()> {
        self.index = readable.try_into()?;
        Ok(())
    }

    /// Serialize the index as we're currently holding it to a Write.
    fn save(self: &mut Self, writeable: &mut (dyn Write)) -> FcResult<()> {
        let blob: Blob = self.index.clone().try_into()?;
        writeable.write(&blob)?;
        Ok(())
    }
}

/// This is the index related interface, providing convenient access
/// to the actual index data. That's its main mission, not persistence or
/// storage/file backend management.
impl IndexProvider for IndexFile {
    fn get_index_ref(self: &mut Self) -> FcResult<&mut Index> {
        Ok(&mut self.index)
    }
    fn set_index(self: &mut Self, index: Index)
    -> FcResult<()> {
        self.index = index;
        Ok(())
    }
}

impl BlobProvider for IndexFile {

    fn clone_blob(&self) -> FcResult<Blob> {
        Ok(self.index.clone().try_into()?)
    }

    fn into_blob(self: Box<Self>) -> FcResult<Blob> {
        Ok(self.index.try_into()?)
    }
    
}

impl Hashable for IndexFile {
    fn get_hash(&self) -> FcResult<String> {
        (self as &(dyn BlobProvider)).get_hash()
    }
}

impl RepoIndexFile for IndexFile {}