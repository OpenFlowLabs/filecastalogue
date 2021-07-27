use std::{io::{BufReader, Read, Write}};
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
        
        let mut serialized = String::new();
        // TODO: If we're using access_repo_file_error!, we should use it for
        //  this too, or not use it at all.
        readable.read_to_string(&mut serialized)?;
        
        match serde_json::from_str(&serialized) {
            Ok(deserialized_file_contents) => {
                self.state = deserialized_file_contents;
                Ok(())
            },
            Err(serde_error) => Err(access_repo_file_error!(
                OffendingAction::LoadingRepoFile,
                context => "Trying to load local state file from the filesystem.",
                identifier => STATE_FILE_NAME,
                wrapped => WrappedError::Serde(serde_error)
            ))
        }
    }

    fn save(self: &mut Self, writeable: &mut dyn Write) -> FcResult<()> {
        // TODO: If we're using access_repo_file_error!, we should use it for
        //  this too, or not use it at all.
        let serialized = serde_json::to_string_pretty(&self.state)?;
        
        match writeable.write(serialized.as_bytes()) {
            Ok(_) => Ok(()),
            Err(io_error) => Err(access_repo_file_error!(
                OffendingAction::SavingRepoFile,
                context => "Trying to save local state file to the filesystem.",
                identifier => STATE_FILE_NAME,
                wrapped => WrappedError::Io(io_error)
            ))
        }
    }
}

impl StateProvider for StateFile {
    fn get_state(self: &mut Self) -> crate::meta::state::model::State {
        todo!()
    }
}