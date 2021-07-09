use std::{io::{BufReader, BufWriter, Read, Write}};
use crate::{access_repo_file_error, error::{Error, ErrorKind,
    FcResult, WrappedError}, files::{AccessRepoFileErrorPayload,
        OffendingAction}, globals::STATE_FILE_NAME};
use crate::{files::{RepoFile, state::StateProvider},
meta::state::model::State};

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
            Err(serde_error) => Err(access_repo_file_error!(
                OffendingAction::LoadingRepoFile,
                context => "Trying to load local state file from the filesystem.",
                identifier => STATE_FILE_NAME,
                wrapped => serde_error
            ))
        }
    }

    fn save(self: &mut Self, writeable: &mut (dyn Write)) -> FcResult<()> {
        let writer = BufWriter::new(writeable);
        match serde_json::to_writer_pretty(writer, &self.state) {
            Ok(_) => Ok(()),
            Err(serde_error) => Err(access_repo_file_error!(
                OffendingAction::SavingRepoFile,
                context => "Trying to save local state file to the filesystem.",
                identifier => STATE_FILE_NAME,
                wrapped => serde_error
            ))
        }
    }
}

impl StateProvider for StateFile {
    fn get_state(self: &mut Self) -> crate::meta::state::model::State {
        todo!()
    }
}