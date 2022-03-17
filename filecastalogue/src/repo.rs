use std::ffi::OsString;
use std::{ffi::OsStr, io::Read};
use crate::error::FcResult;
use crate::files::state_collection::StateFileCollection;
use crate::journal;
use crate::files::index_collection::IndexFileCollection;
use crate::files::tracked_ordinary_blob_collection::TrackedOrdinaryBlobFileCollection;
use crate::meta::file_aspects::aspects::directory::TrackableDirectoryAspects;
use crate::meta::file_aspects::aspects::non_existing::TrackableNonExistingAspects;
use crate::meta::file_aspects::aspects::non_existing::TrackedNonExistingAspects;
use crate::meta::file_aspects::aspects::ordinary::TrackableOrdinaryAspects;
use crate::meta::file_aspects::aspects::symlink::TrackableSymlinkAspects;
use crate::meta::file_aspects::enums::TrackedFileAspects;
use crate::meta::index::accessor::IndexAccessor;
use crate::meta::repo_exported_file_list::model::RepoExportedFileList;
use crate::meta::state::accessor::StateAccessor;
use crate::meta::version::accessor::VersionAccessor;
use crate::meta::version::model::Version;

pub struct Repo<
    // Handler: FiniteStreamHandler,
    StateFile: StateFileCollection,
    Indexes: IndexFileCollection,
    Blobs: TrackedOrdinaryBlobFileCollection,
    Journal: journal::Journal
    >
    {
        pub state_collection: StateFile,
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
    StateCollection: StateFileCollection,
    Indexes: IndexFileCollection,
    Blobs: TrackedOrdinaryBlobFileCollection,
    Journal: journal::Journal
    > Repo<StateCollection, Indexes, Blobs, Journal> {
        pub fn new(
            state_collection: StateCollection,
            indexes: Indexes,
            blobs: Blobs,
            journal: Journal,
        ) -> Repo<StateCollection, Indexes, Blobs, Journal> {
            Repo {
                state_collection,
                indexes: indexes,
                blobs: blobs,
                journal: journal
            }
        }
        pub fn has_version(self: &'rpo mut Self, id: &str) -> FcResult<bool> {
            let mut state_file = self.state_collection.get_state_file()?;
            Ok(state_file.get_state_ref()?.clone().has_version(id))
        }

        pub fn add_version(self: &'rpo mut Self, id: &str)
        -> FcResult<&'rpo mut Self> {
            /*
                Has to do these things:
                    - Add new version to state file.
                    - Make sure index file exists.
            */

            let mut version = Version::new();
            let index_file = self.indexes.create_unwritten_empty_index_file_box();
            let hash = self.indexes.put_index_file(index_file)?;
            // TODO [api]: `&hash`, even though the hash is consumed by `set_index_id`. Either
            //  take a value only or decide whether AsRef is appropriate, or some other sugar.
            version.set_index_id(&hash);
            let mut state_file = self.state_collection.get_state_file()?;
            state_file.get_state_ref()?.add_version(id, version)?;

            // TODO: Saving state?
            self.state_collection.put_state_file(state_file)?;

            Ok(self)
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
            file_path: OsString,
            trackable_aspects: TrackableNonExistingAspects,
        ) -> FcResult<&'rpo mut Self> {
            let mut state_file  = self.state_collection.get_state_file()?;
            let mut version = state_file
                .get_state_ref()?
                .get_version(version_id)?;
            let mut index_file = match version.get_index_id() {
                Some(index_id) => self.indexes.get_index_file(&index_id)?,
                None => self.indexes.create_unwritten_empty_index_file_box()
            };
            
            index_file.get_index_ref()?.track_file(
                file_path, 
                TrackedFileAspects::NonExisting(
                    TrackedNonExistingAspects::from_trackable(trackable_aspects)
                )
            )?;
            let hash = self.indexes.put_index_file(index_file)?;
            version.set_index_id(&hash);
            state_file.get_state_ref()?.put_version(version_id, version);

            // TODO: Saving state?
            self.state_collection.put_state_file(state_file)?;

            Ok(self)
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
        
        pub fn get_files(
            &'rpo mut self,
            version_id: &str,
            file_list: &mut (dyn RepoExportedFileList)
        ) -> FcResult<&'rpo mut Self> {
            let mut state_file = self.state_collection.get_state_file()?;
            let version = state_file
                .get_state_ref()?
                .get_version(version_id)?;
            let index_id = match version.get_index_id() {
                Some(index_id) => index_id,
                // No index, no files to add to the file list.
                None => return Ok(self),
            };
            let mut index_file = self.indexes.get_index_file(&index_id)?;
            let index = index_file.get_index_ref()?;

            for (path, tracked_file_aspects) in &index.files {
                match tracked_file_aspects {
                    
                    TrackedFileAspects::NonExisting(
                        tracked_non_existing_aspects
                    ) => {
                        file_list.add_non_existing(
                            // TODO [clone]: Evaluate and possibly refactor.
                            path.clone(),
                            // TODO [clone]: Evaluate and possibly refactor.
                            tracked_non_existing_aspects.clone()
                        )?;
                    }
                    
                    TrackedFileAspects::Directory(
                        tracked_directory_aspects
                    ) => {
                        file_list.add_directory(
                            // TODO [clone]: Evaluate and possibly refactor.
                            path.clone(),
                            // TODO [clone]: Evaluate and possibly refactor.
                            tracked_directory_aspects.clone()
                        )?;
                    }
                    
                    TrackedFileAspects::Ordinary(
                        tracked_ordinary_aspects
                    ) => {
                        let blob_provider_file = self.blobs.get_file(
                            &tracked_ordinary_aspects.hash
                        )?;
                        let blob_provider =
                            blob_provider_file.as_tracked_ordinary_blob_provider_box();
                        file_list.add_ordinary(
                            // TODO [clone]: Evaluate and possibly refactor.
                            path.clone(),
                            // TODO [clone]: Evaluate and possibly refactor.
                            tracked_ordinary_aspects.clone(),
                            blob_provider
                        )?;
                    }

                    TrackedFileAspects::Symlink(
                        tracked_symlink_aspects
                    ) => {
                        file_list.add_symlink(
                            // TODO [clone]: Evaluate and possibly refactor.
                            path.clone(),
                            // TODO [clone]: Evaluate and possibly refactor.
                            tracked_symlink_aspects.clone()
                        )?;
                    }
                }
            }
            
            Ok(self)
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