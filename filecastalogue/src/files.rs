use std::fmt::{self, Debug, Display};
use crate::error::{FcResult, Payload};

pub mod blob;
pub mod index;
pub mod state;
pub mod blobs;
pub mod indexes;

pub struct AccessRepoFileErrorPayload {
    pub offending_action: OffendingAction,
    pub repo_file_variety: String,
}

impl AccessRepoFileErrorPayload {
    pub fn new<VRef: AsRef<str>>(
        offending_action: OffendingAction,
        repo_file_variety: VRef
    ) -> Self {
        Self {
            offending_action: offending_action,
            repo_file_variety: repo_file_variety.as_ref().to_owned()
        }
    }
}

#[derive(Debug)]
pub enum OffendingAction {
    LoadingRepoFile,
    SavingRepoFile,
}

impl OffendingAction {
    pub(crate) fn as_str(&self) -> &'static str {
        match self {
            Self::LoadingRepoFile => "loading repo-file",
            Self::SavingRepoFile => "saving repo-file"
        }
    }
}

impl Display for OffendingAction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Debug for AccessRepoFileErrorPayload {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Failed {} of the {} variety.",
            self.offending_action,
            self.repo_file_variety,
        )
    }
}

impl Display for AccessRepoFileErrorPayload {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Failed {} of the {} variety.",
            self.offending_action,
            self.repo_file_variety,
        )
    }
}

impl Payload for AccessRepoFileErrorPayload {}

pub trait RepoFile {
    fn load(self: &mut Self) -> FcResult<&mut Self>;
    fn save(self: &mut Self) -> FcResult<&mut Self>;
}

pub trait RepoFileCollection {
    
}

pub trait StateProvider {
    fn get_state(self: &mut Self) -> crate::meta::state::model::State;
}