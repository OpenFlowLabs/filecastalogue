use std::{io::{Read, Write}};
use crate::{access_repo_file_error, error::{Error, ErrorKind,
    FcResult, WrappedError}, files::{AccessRepoFileErrorPayload,
        OffendingAction}, globals::STATE_FILE_NAME};
use crate::{files::{RepoFile, state::StateProvider},
meta::state::model::State, meta::converter::Converter};


pub struct StateFile {

    /// This is where the state is "cached" when it's loaded or set by
    /// other means, and where it will be read from when saving it.
    /// 
    /// Loading the file will write to this, saving the file
    /// will read this (and write it to the file). In general, this is
    /// supposed to provide an in-place "workspace" to modify
    /// the state without persisting every single immediate step.
    /// 
    /// By convention, when setting our .state member from anything else
    /// than an already existing State value, only the principal conversion
    /// methods belonging to the State model should be used to obtain
    /// the new value.
    /// 
    /// Likewise, when converting the state value to something else, such
    /// as a blob, only use the therein contained conversion methods.
    /// 
    /// Using only principal methods for conversions ensures a single source
    /// of process of how the state is transformed between its serialized
    /// and deserialized form, which part of the code is responsible for it
    /// and where to look when things go wrong.
    pub state: State,
}

impl StateFile {

    /// Create a StateFile struct from a blob provided by a Read.
    ///
    /// The blob needs to be JSON deserializable by serde_json.
    pub fn from_existing(readable: &mut (dyn Read)) -> FcResult<Self> {
        Ok(Self {
            state: State::from_read(readable)?
        })
    }
}

/// The interface for getting data from and to persistent storage.
/// 
/// This is supposed to be used by whatever is handling persistent storage
/// for the state file, feeding a Read/Write implementation to the herein
/// implemented methods to provide us with an implementation agnostic
/// interface to the persistent storage.
impl RepoFile for StateFile {

    /// Load the JSON state data from a Read.
    /// 
    /// The mental model is that the Read represents the state's 
    /// persistent storage in JSON form. The data received needs
    /// to be deserializable by serde_json.
    fn load(self: &mut Self, readable: &mut (dyn Read)) -> FcResult<()> {
        
        // TODO: If we're using access_repo_file_error!, we should use it for
        //  this too, or not use it at all.
        match State::from_read(readable) {
            Ok(deserialized_file_contents) => {
                self.state = deserialized_file_contents;
                Ok(())
            },
            Err(error) => Err(access_repo_file_error!(
                OffendingAction::LoadingRepoFile,
                context => "Trying to load local state file from the filesystem.",
                identifier => STATE_FILE_NAME,
                wrapped => WrappedError::Fc(Box::new(error))
            ))
        }
    }

    /// Serialize our current version of the state to a Write.
    fn save(self: &mut Self, writeable: &mut dyn Write) -> FcResult<()> {

        // TODO: If we're using access_repo_file_error!, we should use it for
        //  this too, or not use it at all.
        match writeable.write(&self.state.to_blob()?) {
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