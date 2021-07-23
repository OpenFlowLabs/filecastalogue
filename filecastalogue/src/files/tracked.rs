use super::{RepoFile, blob::BlobProvider};

/* Notes:
    There are two reasons why this exists:
    - Consistency with the "Provider" pattern of other RepoFiles.
      The rest of the world doesn't care that Tracked files are special
      as in that their canonical content is simply their binary blob.
    - It decouples the concern of "Tracked file provides things" from
      "blob files have blobs".
*/
/// Intended for access to the actual binary blob of a Tracked file.
pub trait TrackedBlobProvider: BlobProvider {}

/* Notes:
    Technically, this should probably be named "RepoTrackedBlobFile", with
    everything that's just named "Tracked" getting renamed to "TrackedBlob"
    However, index files'd be affected by that logic too. Currently, "Blob"
    is being dropped in the naming for the sake of brevity.
*/
/// This is the codified representation of a Tracked blob file, a file
/// that's stored in the Repo with the purpose of holding the binary blob
/// of a file we're tracking.
pub trait RepoTrackedFile: RepoFile + TrackedBlobProvider {}