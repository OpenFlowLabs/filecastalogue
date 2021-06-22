use crate::{
    error::FcResult, files::{
        RepoFile,
        index::IndexProvider},
    finite_stream_handlers::FiniteStreamHandler, meta::index::model::Index};

pub trait RepoIndexFile: RepoFile + IndexProvider {}

/**
A local index file.
 */
pub struct IndexFile<Handler> where Handler: FiniteStreamHandler {

    /** The handler provides the actual file operations. */
    pub handler: Handler,

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

impl<Handler: FiniteStreamHandler> IndexFile<Handler> {
    /** Create a new IndexFile.
    The file's contents will immediately be read and cached to
    self.index. */
    pub fn new(handler: Handler) -> FcResult<Self> {
        let mut mut_handler = handler;
        Ok(Self {
            index: mut_handler.read_all()?,
            handler: mut_handler,
        })
    }
    /** Load the index from the file. */
    pub fn load(self: &mut Self) -> FcResult<()> {
        self.index = self.handler.read_all()?;
        Ok(())
    }
    /** Save the index to the file. */
    pub fn save(self: &mut Self) -> FcResult<()> {
        self.handler.write_all(&self.index);
        Ok(())
    }
}

/**
Operations that pertain first and foremost to the actual
file backing of the "IndexFile"; its persistence layer, so to say.
*/
impl<Handler: FiniteStreamHandler> RepoFile for IndexFile<Handler> {
    
    fn load(self: &mut Self) -> FcResult<()> {
        self.load();
        Ok(())
    }

    fn save(self: &mut Self) -> FcResult<()> {
        self.save();
        Ok(())
    }
}

/** This is the index related stuff, providing convenient access
to the actual index data. That's its main mission, not persistence or
storage/file backend management. */
impl<Handler: FiniteStreamHandler> IndexProvider for IndexFile<Handler> {
    fn get_index(self: &mut Self) -> FcResult<&Index> {
        Ok(&self.index)
    }
    fn set_index(self: &mut Self, index: &dyn AsRef<Index>)
    -> FcResult<()> {
        self.index = index.as_ref().to_owned();
        Ok(())
    }
}

impl<Handler: FiniteStreamHandler> RepoIndexFile for IndexFile<Handler> {}