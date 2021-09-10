use serde::{Serialize, Deserialize};

/// Aspects of a symlink relevant when not tracked in a Repo.
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct TrackableSymlinkAspects {
    pub linked_to: String
}

/// Aspects of a symlink relevant when it's tracked in a Repo.
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct TrackedSymlinkAspects {
    pub linked_to: String
}

/// Representation of the tracking of a symlink in a repo when
/// exported from it.
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct RepoExportedSymlinkAspects {
    pub linked_to: String
}

impl TrackableSymlinkAspects {

    pub fn new(linked_to: String) -> Self {
        Self {
            linked_to: linked_to
        }
    }
}

impl TrackedSymlinkAspects {

    pub fn new(linked_to: String) -> Self {
        Self {
            linked_to: linked_to
        }
    }

    pub fn from_trackable(trackable_aspects: TrackableSymlinkAspects) -> Self {
        Self::new(
            trackable_aspects.linked_to
        )
    }
}
