use serde::{Serialize, Deserialize};

/// Aspects relevant when tracking the non-existence of a file in a Repo.
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct TrackedNonExistingAspects {}

/// Aspects of a non-existing file relevant when not being tracked in a repo.
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct TrackableNonExistingAspects {}