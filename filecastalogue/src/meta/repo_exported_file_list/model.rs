use std::{ffi::{OsStr, OsString}, vec};
use crate::{
    error::FcResult,
    files::tracked_ordinary_blob::TrackedOrdinaryBlobProvider,
    meta::{
        blob::repo_exported::RepoExportedHeapOrdinaryBlobProvider,
        file_aspects::{
            aspects::{directory::{RepoExportedDirectoryAspects, TrackedDirectoryAspects},
            non_existing::{RepoExportedNonExistingAspects, TrackedNonExistingAspects},
            ordinary::{RepoExportedOrdinaryAspects, TrackedOrdinaryAspects},
            symlink::{RepoExportedSymlinkAspects, TrackedSymlinkAspects}},
            enums::RepoExportedFileAspects
        }
    }
};

// TODO [doc]: Needs documentation.
// TODO [directory-structure]: Evaluate how well this really fits into `meta`.

pub trait RepoExportedFileList {

    fn consume_as_vec(self: Box<Self>) -> Vec<Box<dyn RepoExportedFile>>;

    fn add_non_existing(
        &mut self,
        path: OsString,
        tracked_aspects: TrackedNonExistingAspects
    ) -> FcResult<&mut dyn RepoExportedFileList>;

    fn add_directory(
        &mut self,
        path: OsString,
        tracked_aspects: TrackedDirectoryAspects
    ) -> FcResult<&mut dyn RepoExportedFileList>;

    fn add_ordinary(
        &mut self,
        path: OsString,
        tracked_aspects: TrackedOrdinaryAspects,
        blob_provider: &dyn TrackedOrdinaryBlobProvider
    ) -> FcResult<&mut dyn RepoExportedFileList>;

    fn add_symlink(
        &mut self,
        path: OsString,
        tracked_aspects: TrackedSymlinkAspects
    ) -> FcResult<&mut dyn RepoExportedFileList>;
}

struct RepoExportedVecFileList {
    vec: Vec<Box<dyn RepoExportedFile>>
}

impl RepoExportedFileList for RepoExportedVecFileList {

    fn consume_as_vec(self: Box<Self>) -> Vec<Box<dyn RepoExportedFile>> {
        self.vec
    }

    fn add_non_existing(
        &mut self,
        path: OsString,
        tracked_aspects: TrackedNonExistingAspects
    ) -> FcResult<&mut dyn RepoExportedFileList> {
        let file: Box<dyn RepoExportedFile> = Box::new(
            RepoExportedHeapFile::new(
                path,
                RepoExportedFileAspects::NonExisting(
                    RepoExportedNonExistingAspects::from_tracked(
                        tracked_aspects
                    )
                )
            )
        );
        self.vec.push(file);
        Ok(self)
    }

    fn add_directory(
        &mut self,
        path: OsString,
        tracked_aspects: TrackedDirectoryAspects
    ) -> FcResult<&mut dyn RepoExportedFileList> {
        let file: Box<dyn RepoExportedFile> = Box::new(
            RepoExportedHeapFile::new(
                path,
                RepoExportedFileAspects::Directory(
                    RepoExportedDirectoryAspects::from_tracked(
                        tracked_aspects
                    )
                )
            )
        );
        self.vec.push(file);
        Ok(self)
    }

    fn add_ordinary(
        &mut self,
        path: OsString,
        tracked_aspects: TrackedOrdinaryAspects,
        blob_provider: &dyn TrackedOrdinaryBlobProvider
    ) -> FcResult<&mut dyn RepoExportedFileList> {
        let file: Box<dyn RepoExportedFile> = Box::new(
            RepoExportedHeapFile::new(
                path,
                RepoExportedFileAspects::Ordinary(
                    RepoExportedOrdinaryAspects::from_tracked(
                        tracked_aspects,
                        Box::new(blob_provider.get_blob()?)
                    )
                )
            )
        );
        self.vec.push(file);
        Ok(self)
    }

    fn add_symlink(
        &mut self,
        path: OsString,
        tracked_aspects: TrackedSymlinkAspects
    ) -> FcResult<&mut dyn RepoExportedFileList> {
        let file: Box<dyn RepoExportedFile> = Box::new(
            RepoExportedHeapFile::new(
                path,
                RepoExportedFileAspects::Symlink(
                    RepoExportedSymlinkAspects::from_tracked(
                        tracked_aspects
                    )
                )
            )
        );
        self.vec.push(file);
        Ok(self)
    }
}

////////////////////////////////////////////////////////////////////////////////
// Initializer
////////////////////////////////////////////////////////////////////////////////

pub trait RepoExportedFileInitializer {
    fn set_from_non_existing(
        &mut self,
        path: &OsStr,
        aspects: RepoExportedNonExistingAspects
    ) -> &dyn RepoExportedFile;
}

impl RepoExportedFileInitializer for RepoExportedHeapFile {
    fn set_from_non_existing(
        &mut self,
        path: &OsStr,
        aspects: RepoExportedNonExistingAspects
    ) -> &dyn RepoExportedFile {
        self.path = path.to_owned();
        self.aspects = RepoExportedFileAspects::NonExisting(aspects);
        self
    }
}

////////////////////////////////////////////////////////////////////////////////
// Item
////////////////////////////////////////////////////////////////////////////////

/// A driver for the `RepoExportedFile` trait which may store
/// data on the heap, specifically the blob of ordinary files.
pub struct RepoExportedHeapFile {
    pub path: OsString,
    pub aspects: RepoExportedFileAspects,
}

impl RepoExportedHeapFile {
    /// Constructs a new `RepoExportedHeapFile`.
    pub fn new(path: OsString, aspects: RepoExportedFileAspects) -> Self {
        Self {
            path,
            aspects
        }
    }
}

pub trait RepoExportedFile {
    fn get_path(&self) -> OsString;
    fn get_aspects(&self) -> &RepoExportedFileAspects;
}

impl RepoExportedFile for RepoExportedHeapFile {
    
    fn get_path(&self) -> OsString {
        self.path.to_owned()
    }
    
    fn get_aspects(&self) -> &RepoExportedFileAspects {
        &self.aspects
    }
}

////////////////////////////////////////////////////////////////////////////////
// Iterators
////////////////////////////////////////////////////////////////////////////////

pub struct IntoIter {
    files: vec::IntoIter<Box<dyn RepoExportedFile>>,
}

impl IntoIter {
    fn new(files: Vec<Box<dyn RepoExportedFile>>)
    -> Self {
        let files = files.into_iter();
        Self {
            files: files,
        }
    }
}

impl Iterator for IntoIter {
    type Item = Box<dyn RepoExportedFile>;

    fn next(&mut self) -> Option<Self::Item> {
        return self.files.next()
    }
}

impl IntoIterator for Box<dyn RepoExportedFileList> {
    type Item = Box<dyn RepoExportedFile>;
    type IntoIter = IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter::new(
            self.consume_as_vec()
        )
    }
}
