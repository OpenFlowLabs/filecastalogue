use serde::{Serialize, Deserialize};
use super::aspects::{
    directory::{RepoExportedDirectoryAspects, TrackedDirectoryAspects},
    // hardlink::{RepoExportedHardlinkAspects, TrackedHardlinkAspects},
    non_existing::{RepoExportedNonExistingAspects, TrackedNonExistingAspects},
    ordinary::{RepoExportedOrdinaryAspects, TrackedOrdinaryAspects},
    symlink::{RepoExportedSymlinkAspects,TrackedSymlinkAspects}
};

/// Encapsulates the various types of file aspects relevant when a file is
/// being tracked in a Repo.
/// The primary use case is describing file aspects in JSON models.
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(
    tag = "kind",
    rename_all(
        serialize = "snake_case",
        deserialize = "snake_case"
    )
)]
pub enum TrackedFileAspects {
    NonExisting(TrackedNonExistingAspects),
    Directory(TrackedDirectoryAspects),
    Ordinary(TrackedOrdinaryAspects),
    Symlink(TrackedSymlinkAspects),
    // Hardlink(TrackedHardlinkAspects) // TODO [maybe]
}