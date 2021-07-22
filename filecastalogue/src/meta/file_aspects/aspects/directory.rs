use serde::{Serialize, Deserialize};
use super::super::attributes::Attributes;

/// Aspects of a directory relevant when it's tracked in a Repo.
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct TrackedDirectoryAspects {
    pub attributes: Attributes
}

/// Aspects of a directory relevant when not tracked in a Repo.
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct TrackableDirectoryAspects {
    pub attributes: Attributes
}