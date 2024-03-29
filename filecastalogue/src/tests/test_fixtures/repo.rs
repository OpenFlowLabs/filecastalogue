use std::ffi::OsString;
use crate::error::FcResult;
use crate::files::index_collection::MiscIndexFileCollection;
use crate::files::state_collection::MiscStateFileCollection;
use crate::files::tracked_ordinary_blob_collection::MiscTrackedOrdinaryBlobFileCollection;
use crate::globals::STATE_FILE_NAME;
use crate::journal::OptimisticDummyJournal;
use crate::opaque_collection_handler::drivers::local::LocalDir;
use crate::repo::Repo;

use super::super::TEST_CONF;

pub(crate) const NON_EXISTING_VERSION_INDEX: usize = 1;
pub(crate) const ADDED_VERSION_INDEX: usize = 100;
pub(crate) const MINIMAL_REPO_PARENT_PATH: &str = ".";

pub(in crate::tests) fn create_minimal_repo_struct(test_id: &str)
-> FcResult<Repo<
    MiscStateFileCollection<LocalDir>,
    MiscIndexFileCollection<LocalDir>,
    MiscTrackedOrdinaryBlobFileCollection<LocalDir>,
    OptimisticDummyJournal
>> {
    TEST_CONF::MINIMAL_REPO_SITE.set_up(test_id)?;
    Ok(Repo::new(
        // TODO 1: Create mock state file.
        MiscStateFileCollection::new(LocalDir::new(
            TEST_CONF::MINIMAL_REPO_SITE.get_repo_path(test_id)?), OsString::from(STATE_FILE_NAME)),
        // TODO 2: Create mock index collection.
        MiscIndexFileCollection::new(LocalDir::new(
            TEST_CONF::MINIMAL_REPO_SITE.get_blob_dir_path(test_id)?)),
        // TODO 3: Create mock blobs collection.
        MiscTrackedOrdinaryBlobFileCollection::new(
            LocalDir::new(
                TEST_CONF::MINIMAL_REPO_SITE.get_blob_dir_path(test_id)?)),
        OptimisticDummyJournal::new(),
    ))
}