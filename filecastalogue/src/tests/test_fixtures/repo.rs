use crate::{error::{FcResult}, files::{
        tracked_ordinary_blob_collection::MiscTrackedOrdinaryBlobFileCollection,
        index_collection::MiscIndexFileCollection,
        state::StateFile},
    journal::OptimisticDummyJournal,
    opaque_collection_handlers::LocalDir, repo::Repo};
use super::super::TEST_CONF;

pub(crate) const NON_EXISTENT_VERSION_ID: &str = "0";
pub(crate) const MINIMAL_REPO_PARENT_PATH: &str = ".";

pub(in crate::tests) fn create_minimal_repo_struct()
-> FcResult<Repo<
    StateFile,
    MiscIndexFileCollection<LocalDir>,
    MiscTrackedOrdinaryBlobFileCollection<LocalDir>,
    OptimisticDummyJournal
>> {
    TEST_CONF::MINIMAL_REPO_SITE.set_up()?;
    Ok(Repo::new(
        // TODO 1: Create mock state file.
        StateFile::from_existing(
            &mut TEST_CONF::MINIMAL_REPO_SITE.get_state_readable()?
        )?,
        // TODO 2: Create mock index collection.
        MiscIndexFileCollection::new(LocalDir::new(
            TEST_CONF::MINIMAL_REPO_SITE.get_blob_dir_path()?)),
        // TODO 3: Create mock blobs collection.
        MiscTrackedOrdinaryBlobFileCollection::new(
            LocalDir::new(
                TEST_CONF::MINIMAL_REPO_SITE.get_blob_dir_path()?)),
        OptimisticDummyJournal::new(),
    ))
}