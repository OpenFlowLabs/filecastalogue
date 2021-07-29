use std::{ffi::OsStr, io::Read};

use crate::{error::FcResult, files::{RepoFile, tracked_ordinary_blob_collection::TrackedOrdinaryBlobFileCollection, 
    index_collection::IndexFileCollection, state::StateProvider}, journal, meta::{file_aspects::aspects::{directory::TrackableDirectoryAspects, non_existing::TrackableNonExistingAspects, ordinary::TrackableOrdinaryAspects, symlink::TrackableSymlinkAspects}, state::accessor::Accessor, version::{accessor::VersionAccessor, model::Version}}};

pub struct Repo<
    // Handler: FiniteStreamHandler,
    StateFile: RepoFile + StateProvider,
    Indexes: IndexFileCollection,
    Blobs: TrackedOrdinaryBlobFileCollection,
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
    'rpo,
    // Handler: FiniteStreamHandler,
    StateFile: RepoFile + StateProvider,
    Indexes: IndexFileCollection,
    Blobs: TrackedOrdinaryBlobFileCollection,
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
        pub fn has_version(self: &'rpo mut Self, id: &str) -> FcResult<bool> {
            Ok(self.state_file.get_state()?.clone().has_version(id))
        }
        // version is consumed here, in order to force explicit handling of
        // situations where the version has to continue being available in the
        // calling context.
        pub fn add_version(self: &'rpo mut Self, id: &str, version: Version)
        -> FcResult<&'rpo mut Self> {
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

        /// Track a file that doesn't exist.
        /// 
        /// WARNING: This informs tools applying this tracked entry to
        /// make sure no file doesn't exist at that path on the target
        /// system, e.g. they might delete anything found at that path.
        /// 
        /// Takes the ID of the version that should be tracking it and
        /// the path where nothing is supposed to exist on the target system.
        pub fn track_non_existing(
            &'rpo mut self,
            version_id: &str,
            file_path: &OsStr,
            // trackable_aspects: TrackableNonExistingAspects,
        ) -> FcResult<&'rpo mut Self> {
            todo!();
        }

        /// Track a directory.
        /// 
        /// Takes the ID of the version that should be tracking it, the
        /// path of the directory on the tracked system as well as an object
        /// describing the aspects it should have there.
        pub fn track_directory(
            &'rpo mut self,
            version_id: &str,
            file_path: &OsStr,
            trackable_aspects: TrackableDirectoryAspects,
        ) -> FcResult<&'rpo mut Self> {
            todo!();
        }

        /// Track an ordinary (blob) file.
        /// 
        /// Takes the ID of the version that should be tracking it, the
        /// path of the file on the tracked system as well as an object
        /// describing the aspects it (should) have there and a Read providing
        /// its blob.
        pub fn track_ordinary(
            &'rpo mut self,
            version_id: &str,
            file_path: &OsStr,
            trackable_aspects: TrackableOrdinaryAspects,
            blob_readable: &(dyn Read)
        ) -> FcResult<&'rpo mut Self> {
            todo!();
        }

        /// Track a symlink.
        /// 
        /// Takes the ID of the version that should be tracking it, the
        /// path of the symlink on the tracked system as well as an object
        /// describing the aspects it (should) have there.
        pub fn track_symlink(
            &'rpo mut self,
            version_id: &str,
            file_path: &OsStr,
            trackable_aspects: TrackableSymlinkAspects,
        ) -> FcResult<&'rpo mut Self> {
            todo!();
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