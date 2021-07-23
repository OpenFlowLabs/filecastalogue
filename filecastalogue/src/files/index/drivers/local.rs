use std::io::{BufReader, Read, Write};
use crate::error::FcResult;
use crate::files::hashable::Hashable;
use super::super::super::{RepoFile, blob::BlobProvider, index::IndexProvider};
use super::super::Index;
pub trait RepoIndexFile: RepoFile + IndexProvider + BlobProvider + Hashable {}

/**
A local index file.
 */
pub struct IndexFile {

    /** This is where the index is "cached" when it's loaded from
    the file or set by other means, and where it will be read from
    when saving the index to the file.
    Loading the file will write to this, saving the file
    will read this (and write it to the file). This might
    also get modified by other means. In general, this is
    supposed to provide an in-place "workspace" to modify
    the index without persisting every single immediate step.
    Other precautions will have to be taken in order to make
    sure this doesn't get messy.*/
    pub index: Index,
}

impl IndexFile {
    /** Create an IndexFile struct from an existing file. */
    pub fn from_existing(readable: &mut (dyn Read)) -> FcResult<Self> {
        let reader = BufReader::new(readable);
        Ok(Self {
            index: serde_json::from_reader(reader)?,
        })
    }
}

/**
Operations that pertain first and foremost to the actual
file backing of the "IndexFile"; its persistence layer, so to say.
*/
impl RepoFile for IndexFile {
    
    fn load(self: &mut Self, readable: &mut (dyn Read)) -> FcResult<()> {
        let reader = BufReader::new(readable);
        self.index = serde_json::from_reader(reader)?;
        Ok(())
    }

    fn save(self: &mut Self, writeable: &mut (dyn Write)) -> FcResult<()> {
        serde_json::to_writer_pretty(writeable, &self.index)?;
        Ok(())
    }
}

/** This is the index related stuff, providing convenient access
to the actual index data. That's its main mission, not persistence or
storage/file backend management. */
impl IndexProvider for IndexFile {
    fn get_index(self: &mut Self) -> FcResult<&Index> {
        Ok(&self.index)
    }
    fn set_index(self: &mut Self, index: &dyn AsRef<Index>)
    -> FcResult<()> {
        self.index = index.as_ref().to_owned();
        Ok(())
    }
}

impl BlobProvider for IndexFile {
    fn get_blob(&self) -> FcResult<Vec<u8>> {
        Ok(serde_json::to_vec_pretty(&self.index)?)
    }
}

impl Hashable for IndexFile {
    fn get_hash(&self) -> FcResult<String> {
        (self as &(dyn BlobProvider)).get_hash()
    }
}

impl RepoIndexFile for IndexFile {}