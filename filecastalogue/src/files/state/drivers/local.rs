use crate::files::RepoFile;

pub struct LocalStateFile {}

impl RepoFile for LocalStateFile {
    fn open(self: &mut Self) -> Result<&mut Self, crate::files::OpenRepoFileError> {
        todo!()
    }

    fn save(self: &mut Self) -> Result<&mut Self, crate::files::SaveRepoFileError> {
        todo!()
    }
}