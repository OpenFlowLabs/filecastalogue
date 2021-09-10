use serde::{Serialize, Deserialize};

// TODO: Evaluate and (perhaps) implement hard link support.

/// Aspects of a hard link relevant when it's tracked in a Repo.
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct TrackedHardlinkAspects {
    // TODO
}

/// Aspects of a hard link relevant when not tracked in a Repo.
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct TrackableHardlinkAspects {
    // TODO
}

/// Representation of the tracking of a hardlink in a repo when
/// exported from it.
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct RepoExportedHardlinkAspects {
    // TODO
}
