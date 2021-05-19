use crate::{
    files::{
        RepoFile, indexes::IndexFileCollection, blobs::BlobFileCollection
    },
    journal
};

pub struct Repo<
    State: RepoFile,
    Indexes: IndexFileCollection,
    Blobs: BlobFileCollection,
    Journal: journal::Journal
    > {
        pub state: State,
        pub indexes: Indexes,
        pub blobs: Blobs,
        pub journal: Journal
    }

impl<
    State: RepoFile,
    Indexes: IndexFileCollection,
    Blobs: BlobFileCollection,
    Journal: journal::Journal
    > Repo<State, Indexes, Blobs, Journal> {
        pub fn new(
            state: State,
            indexes: Indexes,
            blobs: Blobs,
            journal: Journal,
        ) -> Repo<State, Indexes, Blobs, Journal> {
            Repo {
                state: state,
                indexes: indexes,
                blobs: blobs,
                journal: journal
            }
        }
    }