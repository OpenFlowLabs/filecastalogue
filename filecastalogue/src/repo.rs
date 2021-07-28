use crate::{error::FcResult, files::{RepoFile, tracked_collection::TrackedFileCollection, 
    index_collection::IndexFileCollection, state::StateProvider}, journal, meta::{state::accessor::Accessor, version::{accessor::VersionAccessor, model::Version}}};

pub struct Repo<
    // Handler: FiniteStreamHandler,
    StateFile: RepoFile + StateProvider,
    Indexes: IndexFileCollection,
    Blobs: TrackedFileCollection,
    Journal: journal::Journal
    >
    {
        pub state_file: StateFile,
        pub indexes: Indexes,
        pub blobs: Blobs,
        pub journal: Journal
    }

// TODO: Change e.g. state to only be a file-thing and load()
// to return the struct, which we will then set to state.
// Or we leave load() the way it is, make a get() method
// and then decide for ourselves what to do here?
// Implement load/save wrappers on repo.

// TODO: Change RepoFile to a struct which takes handler.
// The driver should probably become trait based - simply put:
// Flip the design around, with RepoFile being the "interface"
// which has a .data property, and the trait being the
// "driver"?

impl<
    // Handler: FiniteStreamHandler,
    StateFile: RepoFile + StateProvider,
    Indexes: IndexFileCollection,
    Blobs: TrackedFileCollection,
    Journal: journal::Journal
    > Repo<StateFile, Indexes, Blobs, Journal> {
        pub fn new(
            state_file: StateFile,
            indexes: Indexes,
            blobs: Blobs,
            journal: Journal,
        ) -> Repo<StateFile, Indexes, Blobs, Journal> {
            Repo {
                state_file: state_file,
                indexes: indexes,
                blobs: blobs,
                journal: journal
            }
        }
        pub fn has_version(self: &mut Self, id: &str) -> FcResult<bool> {
            Ok(self.state_file.get_state()?.clone().has_version(id))
        }
        // version is consumed here, in order to force explicit handling of
        // situations where the version has to continue being available in the
        // calling context.
        pub fn add_version(self: &mut Self, id: &str, version: Version)
        -> FcResult<&mut Self> {
            self.state_file.get_state()?.clone().add_version(id, version.clone())?;
            /*
                Has to do these things:
                    - Add new version to state file.
                    - Make sure index file exists.
            */
            
            let index_id = match version.get_index_id() {
                Some(index_id) => index_id,
                None => todo!(),
            };
            if self.indexes.has_index(&index_id)? {
                todo!()
            }
            else {
                todo!()
            }
        }
    }

// trait Accessor {

// }

// impl<
//     State: RepoFile,
//     Indexes: IndexFileCollection,
//     Blobs: BlobFileCollection,
//     Journal: journal::Journal
// > Accessor for Repo<State, Indexes, Blobs, Journal> {
//     fn add_version(self: &mut Self, id: &str, index: &str) {
//         self.state.
//     }
// }