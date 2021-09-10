use serde::{Serialize, Deserialize};

/// Aspects of a non-existing file relevant when not being tracked in a repo.
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct TrackableNonExistingAspects {}

/// Aspects relevant when tracking the non-existence of a file in a Repo.
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct TrackedNonExistingAspects {}
