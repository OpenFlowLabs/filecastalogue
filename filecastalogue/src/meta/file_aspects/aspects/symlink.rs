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
