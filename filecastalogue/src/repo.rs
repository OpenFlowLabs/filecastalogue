use crate::{files::{
        RepoFile, indexes::IndexFileCollection, blobs::BlobFileCollection
    }, journal, meta::state::model::State};

pub struct RepoState<
    StateFile: RepoFile
    > {
        pub file: StateFile,
        pub data: State
    }

impl<StateFile: RepoFile> RepoState<StateFile> {
    pub fn new(state_file: StateFile) -> Self {
        Self {
            data: state_file.read(),
            file: state_file,
        }
    }
}

pub struct Repo<
    StateFile: RepoFile,
    Indexes: IndexFileCollection,
    Blobs: BlobFileCollection,
    Journal: journal::Journal
    > {
        pub state: RepoState<StateFile>,
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
    StateFile: RepoFile,
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
                state: RepoState::new(state_file),
                indexes: indexes,
                blobs: blobs,
                journal: journal
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