use crate::{error::FcResult,
    files::{blobs::drivers::local::LocalBlobFileCollection,
        indexes::drivers::local::LocalIndexFileCollection,
        state::drivers::local::StateFile},
        finite_stream_handlers::JsonStream,
        journal::OptimisticDummyJournal,
        opaque_collection_handlers::LocalDir,
        repo::Repo};

pub(crate) const NON_EXISTENT_VERSION_ID: &str = "0";

pub(in crate::tests) fn create_minimal_repo_struct()
-> FcResult<Repo<
    StateFile<JsonStream>,
    LocalIndexFileCollection<LocalDir>,
    LocalBlobFileCollection<LocalDir>,
    OptimisticDummyJournal
>> {
    Ok(Repo::new(
        // TODO 1: Create mock state file.
        todo!(),
        // TODO 2: Create mock index collection.
        todo!(),
        // TODO 3: Create mock blobs collection.
        todo!(),
        OptimisticDummyJournal::new(),
    ))
}