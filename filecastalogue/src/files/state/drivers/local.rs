use std::{io::{BufReader, Read, Write}};
use crate::{error::{Error, ErrorKind, FcResult, WrappedError}, files::{AccessRepoFileErrorPayload, OffendingAction}};
use crate::{files::{RepoFile, state::StateProvider},meta::state::model::State};

pub struct StateFile {
    pub state: State,
}

impl StateFile {
    pub fn from_existing(readable: &mut (dyn Read)) -> FcResult<Self> {
        Ok(Self {
            state: serde_json::from_reader(BufReader::new(readable))?,
        })
    }
}

impl RepoFile for StateFile {
    fn load(self: &mut Self, readable: &mut (dyn Read)) -> FcResult<()> {
        let reader = BufReader::new(readable);
        match serde_json::from_reader(reader) {
            Ok(deserialized_file_contents) => {
                self.state = deserialized_file_contents;
                Ok(())
            },
            Err(io_error) => Err(error!(
                ErrorKind::RepoFileOperationFailed,
                "Trying to get deserialized file contents from the handler.",
                AccessRepoFileErrorPayload::new(
                    OffendingAction::LoadingRepoFile,
                    "StateFile"
                ),
                WrappedError::Serde(io_error)
            ))
        }
    }

    fn save(self: &mut Self, writer: &mut (dyn Write)) -> FcResult<()> {
        serde_json::to_writer_pretty(writer, &self.state)?;
        Ok(())
    }
}

impl StateProvider for StateFile {
    fn get_state(self: &mut Self) -> crate::meta::state::model::State {
        todo!()
    }
}