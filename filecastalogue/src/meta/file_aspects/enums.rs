use serde::{Serialize, Deserialize};
use super::aspects::{
    directory::TrackedDirectoryAspects,
    non_existing::TrackedNonExistingAspects,
    ordinary::TrackedOrdinaryAspects,
    symlink::TrackedSymlinkAspects,
    // hardlink::TrackedHardlinkAspects
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
    File(TrackedOrdinaryAspects),
    Symlink(TrackedSymlinkAspects),
    // Hardlink(HardlinkFileKind) // TODO
}