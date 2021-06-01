use std::{fs::File, io::{self, BufReader}, path::Path};
use crate::{error::{Error, FcResult}, files::{AccessRepoFileErrorPayload, OffendingAction}};
use io::{Result as IoResult};
use crate::error::ErrorKind::RepoFileOperationFailed;
use crate::{files::{RepoFile, StateProvider},
finite_stream_handlers::FiniteStreamHandler, meta::state::model::State};

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
    pub fn new(handler: Handler) -> IoResult<Self> {
        let mut mut_handler = handler;
        Ok(Self {
            state: mut_handler.read_all()?,
            handler: mut_handler,
        })
    }
}

impl<Handler: FiniteStreamHandler> RepoFile for StateFile<Handler> {
    fn load(self: &mut Self) -> FcResult<&mut Self> {
        match self.handler.read_all() {
            Ok(deserialized_file_contents) => {
                self.state = deserialized_file_contents;
                Ok(self)
            },
            Err(io_error) => Err(Error::new(
                RepoFileOperationFailed,
                "Trying to get deserialized file contents from the handler.",
                Some(Box::new(AccessRepoFileErrorPayload::new(
                    OffendingAction::LoadingRepoFile,
                    "StateFile"
                ))),
                Some(Box::new(io_error))
            ))
        }
    }

    fn save(self: &mut Self) -> FcResult<&mut Self> {
        todo!()
    }
}

impl<Handler: FiniteStreamHandler> StateProvider for StateFile<Handler> {
    fn get_state(self: &mut Self) -> crate::meta::state::model::State {
        todo!()
    }
}