use std::fmt;

pub mod blob;
pub mod index;
pub mod state;
pub mod blobs;
pub mod indexes;

pub struct OpenRepoFileError {
    pub path: String,
    pub repo_file_variety: String,
    pub context_description: String,
}

impl fmt::Debug for OpenRepoFileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Error: Failed opening repo-file of the {} variety. Path: \"{}\". Context: {}",
            self.path,
            self.repo_file_variety,
            self.context_description
        )
    }
}

impl fmt::Display for OpenRepoFileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Error: Failed opening repo-file of the {} variety. Path: \"{}\".",
            self.repo_file_variety,
            self.path
        )
    }
}

pub struct SaveRepoFileError {
    pub path: String,
    pub repo_file_variety: String,
    pub context_description: String,
}

impl fmt::Debug for SaveRepoFileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Error: Failed saving repo-file of the {} variety. Path: \"{}\". Context: {}",
            self.path,
            self.repo_file_variety,
            self.context_description
        )
    }
}

impl fmt::Display for SaveRepoFileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Error: Failed saving repo-file of the {} variety. Path: \"{}\".",
            self.repo_file_variety,
            self.path
        )
    }
}

pub trait RepoFile {
    fn load(self: &mut Self) -> Result<&mut Self, OpenRepoFileError>;
    fn save(self: &mut Self) -> Result<&mut Self, SaveRepoFileError>;
}

pub trait RepoFileCollection {
    
}

pub trait StateProvider {
    fn get_state(self: &mut Self) -> crate::meta::state::model::State;
}