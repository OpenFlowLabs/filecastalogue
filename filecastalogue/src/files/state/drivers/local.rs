use std::{ffi::{OsStr, OsString}, fs::File, io::{self, BufReader}, path::Path};
use crate::{files::RepoFile, meta::state::model::State};

pub fn file_reader<PathRef: AsRef<Path>>(path: PathRef)
-> Result<BufReader<File>, io::Error> {
    let file = File::open(path)?;
    Ok(BufReader::new(file))
}

pub struct LocalStateFile {
    path: OsString,
    state: State
}

impl LocalStateFile {
    pub fn new(path: &OsStr) -> Result<Self, io::Error> {
        let reader = file_reader(path)?;
        let state_struct = serde_json::from_reader(reader)?;
        Ok(Self {
            path: path.to_owned(),
            state: state_struct
        })
    }
}

impl RepoFile for LocalStateFile {
    fn open(self: &mut Self) -> Result<&mut Self, crate::files::OpenRepoFileError> {
        todo!()
    }

    fn save(self: &mut Self) -> Result<&mut Self, crate::files::SaveRepoFileError> {
        todo!()
    }
}