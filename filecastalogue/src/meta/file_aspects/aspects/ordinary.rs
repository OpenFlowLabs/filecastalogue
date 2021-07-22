use serde::{Serialize, Deserialize};
use super::super::attributes::Attributes;

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