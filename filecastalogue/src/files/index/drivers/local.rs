use std::io::{BufReader, Read, Write};
use crate::{
    error::FcResult, files::{
        RepoFile,
        index::IndexProvider},
    meta::index::model::Index};

pub trait RepoIndexFile: RepoFile + IndexProvider {}

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
        let mut reader = BufReader::new(readable);
        Ok(Self {
            index: serde_json::from_reader(reader)?,
        })
    }
    /** Load the index from the specified source. */
    pub fn load(self: &mut Self, readable: &mut(dyn Read)) -> FcResult<()> {
        let mut reader = BufReader::new(readable);
        self.index = serde_json::from_reader(reader)?;
        Ok(())
    }
    /** Write the index to the specified sink. */
    pub fn save(self: &mut Self, writer: &mut(dyn Write)) -> FcResult<()> {
        serde_json::to_writer_pretty(writer, &self.index);
        Ok(())
    }
}

/**
Operations that pertain first and foremost to the actual
file backing of the "IndexFile"; its persistence layer, so to say.
*/
impl RepoFile for IndexFile {
    
    fn load(self: &mut Self, reader: &mut (dyn Read)) -> FcResult<()> {
        self.load(reader)?;
        Ok(())
    }

    fn save(self: &mut Self, writer: &mut (dyn Write)) -> FcResult<()> {
        self.save(writer)?;
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

impl RepoIndexFile for IndexFile{}