use std::{fs::File, io::{self, BufReader}, path::Path};
use crate::{files::{RepoFile, StateProvider}, finite_stream_handlers::FiniteStreamHandler, meta::state::model::State};

pub fn file_reader<PathRef: AsRef<Path>>(path: PathRef)
-> Result<BufReader<File>, io::Error> {
    // let file = File::open(path)?;
    Ok(BufReader::new(File::open(path)?))
}

pub struct StateFile<Handler> where Handler: FiniteStreamHandler {
    pub handler: Handler,
    pub state: State,
}

impl<Handler: FiniteStreamHandler> StateFile<Handler> {
    pub fn new(handler: Handler) -> Result<Self, io::Error> {
        let mut mut_handler = handler;
        Ok(Self {
            state: mut_handler.read_all()?,
            handler: mut_handler,
        })
    }
}

impl<Handler: FiniteStreamHandler> RepoFile for StateFile<Handler> {
    fn load(self: &mut Self) -> Result<&mut Self, crate::files::OpenRepoFileError> {
        todo!()
    }

    fn save(self: &mut Self) -> Result<&mut Self, crate::files::SaveRepoFileError> {
        todo!()
    }
}

impl<Handler: FiniteStreamHandler> StateProvider for StateFile<Handler> {
    fn get_state(self: &mut Self) -> crate::meta::state::model::State {
        todo!()
    }
}