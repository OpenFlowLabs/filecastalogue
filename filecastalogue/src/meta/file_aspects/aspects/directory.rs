use serde::{Serialize, Deserialize};
use super::super::attributes::Attributes;

/// Aspects of a directory relevant when not tracked in a Repo.
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct TrackableDirectoryAspects {
    pub attributes: Attributes
}

/// Aspects of a directory relevant when it's tracked in a Repo.
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct TrackedDirectoryAspects {
    pub attributes: Attributes
}

/// Representation of the tracking of a directory in a repo when
/// exported from it.
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct RepoExportedDirectoryAspects {
    pub attributes: Attributes
}

impl TrackableDirectoryAspects {

    pub fn new(attributes: Attributes) -> Self {
        Self {
            attributes
        }
    }
}

impl TrackedDirectoryAspects {

    pub fn new(attributes: Attributes) -> Self {
        Self {
            attributes
        }
    }

    pub fn from_trackable(trackable_aspects: TrackableDirectoryAspects) -> Self {
        Self::new(
            trackable_aspects.attributes
        )
    }
}

impl RepoExportedDirectoryAspects {

    pub fn new(attributes: Attributes) -> Self {
        Self {
            attributes
        }
    }

    pub fn from_tracked(tracked_aspects: TrackedDirectoryAspects) -> Self {
        Self::new(
            tracked_aspects.attributes
        )
    }
}
