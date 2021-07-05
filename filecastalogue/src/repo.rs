use crate::{error::FcResult, files::{RepoFile, blobs::BlobFileCollection, 
    indexes::IndexFileCollection, state::StateProvider}, journal, meta::state::accessor::Accessor};

pub struct Repo<
    // Handler: FiniteStreamHandler,
    StateFile: RepoFile + StateProvider,
    Indexes: IndexFileCollection,
    Blobs: BlobFileCollection,
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
    Blobs: BlobFileCollection,
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
        pub fn has_version(self: &mut Self, id: &str) -> bool {
            self.state_file.get_state().has_version(id)
        }
        pub fn add_version(self: &mut Self, id: &str, index: &str)
        -> FcResult<&mut Self> {
            self.state_file.get_state().add_version(id, index)?;
            /*
                Has to do these things:
                    - Add new version to state file.
                    - Make sure index file exists.
            */
                
            if self.indexes.has_index(index)? {
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