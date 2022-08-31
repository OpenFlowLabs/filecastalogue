use serde::{Serialize, Deserialize};
use crate::meta::blob::repo_exported::RepoExportedOrdinaryBlobProvider;

use super::super::attributes::Attributes;

/* Notes:
    Leaving the .hash out for the Trackable model enables a separation of
    concern regarding the hash: The Repo needs the hash and will need all
    the pieces to obtain it (e.g. the file's binary data - which it'll
    need anyway in order to actually track anything useful in the case of
    an ordinary file, regardless of its hash needs), however, other actors
    in the system might be completely oblivious not just to the hash, but
    even the file's binary data itself - which, hypothetically, might arrive
    through a completely different channel (e.g. a dedicated file server)
    than the aspects (e.g. some JSON from some other tool).

    A lot of future flexibility is added by uncoupling this concern.

    It also enables a cleaner library interface free of this potentially
    irrelevant concern for its consumers, tighter scoping of the
    hashing related code bits whilst still providing a means of explicitly
    codifying a model for file aspects in procedures outside the scope
    of the repo.
*/
/// Aspects of an ordinary file relevant when not tracked in a Repo, which
/// for example, is the case when an actor is preparing data for consumption
/// by a Repo, a stage at which not all aspects the Repo might consider
/// relevant might need to be sorted and available right away.
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct TrackableOrdinaryAspects {
    pub attributes: Attributes
}

/// Aspects of an ordinary file relevant when it's tracked in a Repo.
/// Besides their attributes, we're only interested in them in their binary form,
/// particularly the hash of it, which is also what differentiates the Tracked
/// model from the Trackable one: The Repo is concerned with the hash, whilst
/// other actors in the filecastalogue ecosystem might not be.
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct TrackedOrdinaryAspects {
    pub hash: String,
    pub attributes: Attributes
}

/// Representation of the Aspects of an ordinary file when exported from a
/// Repo. Here, the concern is the full and bundled availability of
/// all the pieces required to fully observe the file's state, namely its
/// attributes, and if it's an `ordinary` file, its blob.
///
/// It also sports `repo_blob_hash`, which is the hash of the blob as the
/// Repo knows it. This allows tooling consuming this data to compare what
/// it received to the Repo's truth.
///
/// That concern is separated from `hash` in `TrackedOrdinaryAspects` by
/// design. This may only ever be a one way conversion, from
/// `hash` of `TrackedOrdinaryAspects` to `repo_blob_hash` of this here
/// struct, NEVER the other way around. A Repo's hash is ever only
/// to be calculated by the Repo's own principal means for that particular
/// concern.
pub struct RepoExportedOrdinaryAspects {
    pub repo_blob_hash: String,
    pub attributes: Attributes,
    // pub blob: Box<dyn RepoExportedBlob>
    // This allows for the implementation of non-memory blob storage, e.g.
    // for caching purposes or partial reads (say, when implementing
    // a custom serializer or a custom way to network-transfer blobs).
    pub blob_provider: Box<dyn RepoExportedOrdinaryBlobProvider>
}

impl TrackableOrdinaryAspects {
    pub fn new(attributes: Attributes) -> Self {
        Self {
            attributes: attributes
        }
    }
}

impl TrackedOrdinaryAspects {

    pub fn new(hash: &str, attributes: Attributes) -> Self {
        Self {
            hash: hash.to_owned(),
            attributes: attributes,
        }
    }

    /// Construct from trackable aspects and a hash.
    /// 
    /// Combines the trackable aspects and the hash into an object that
    /// holds all aspects, including the hash we need to track an ordinary
    /// file in a Repo.
    pub fn from_trackable(trackable_aspects: TrackableOrdinaryAspects, hash: &str)
    -> Self {
        Self::new(
            hash,
            trackable_aspects.attributes
        )
    }
}

/* The typing of the `blob` parameters is already of our type, not
 * of its tracked counter part, which deviates from the general
 * convention applied to the other "Aspects" struct's `new` and
 *`from_tracked` methods. That's because it's primarily designed
 * with something wrapping it in mind that'll already handle
 * all the blob matters, something that lends itself more easily
 * to getting (re-)implemented downstream.
 */
impl RepoExportedOrdinaryAspects {

    pub fn new(
        repo_blob_hash: &str,
        attributes: Attributes,
        blob_provider: Box<dyn RepoExportedOrdinaryBlobProvider>
    )-> Self {
        Self::new(
            repo_blob_hash,
            attributes,
            blob_provider
        )
    }

    pub fn from_tracked(
        tracked_aspects: TrackedOrdinaryAspects,
        blob_provider: Box<dyn RepoExportedOrdinaryBlobProvider>
    ) -> Self {
        Self::new(
            &tracked_aspects.hash,
            tracked_aspects.attributes,
            blob_provider
        )
    }
}
