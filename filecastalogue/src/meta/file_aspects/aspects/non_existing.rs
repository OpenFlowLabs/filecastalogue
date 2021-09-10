use serde::{Serialize, Deserialize};

/// Aspects of a non-existing file relevant when not being tracked in a repo.
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct TrackableNonExistingAspects {}

/// Aspects relevant when tracking the non-existence of a file in a Repo.
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct TrackedNonExistingAspects {}

/// Representation of the tracking of a non-existing file in a repo when
/// exported from it.
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct RepoExportedNonExistingAspects {}

impl TrackableNonExistingAspects {

    pub fn new() -> Self {
        Self {}
    }

    pub fn from_tracked(tracked_aspects: TrackableNonExistingAspects) -> Self {
        Self::new()
    }
}

impl TrackedNonExistingAspects {

    pub fn new() -> Self {
        Self {}
    }
}

impl RepoExportedNonExistingAspects {

    pub fn new() -> Self {
        Self {}
    }

    pub fn from_tracked(tracked_aspects: TrackedNonExistingAspects) -> Self {
        Self::new()
    }
}
