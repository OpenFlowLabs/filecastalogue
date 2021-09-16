use crate::{error::FcResult, meta::blob::model::Blob};
use super::hashable::Hashable;

/// Intended for access to the actual binary of either a tracked
/// or an index file, e.g. for hashing or applying state.
/// It's also used to provide access to the blob packaged or
/// referenced in the data returned from file getter functions
/// of Repo.
pub trait BlobProvider {
    fn clone_blob(&self) -> FcResult<Blob>;
    fn into_blob(self: Box<Self>) -> FcResult<Blob>;
}

/*
    'a relaxes the implicit 'static of the trait object, enabling the
    use of Hashable features on RepoIndexFile objects with less than
    'static lifetime.
*/
/// We have blobs and hashes of those blobs. As such, hashing is really
/// a strongly blob related concern for us. To keep hashing related code
/// from sprawling, this serves as a single source of process for how
/// to hash a BlobProvider's blob.
/// 
/// Example:
/// ```
/// use filecastalogue::{
///     error::FcResult,
///     meta::blob::model::Blob,
///     files::{hashable::Hashable, index::IndexFile, blob::BlobProvider}
/// };
/// 
/// struct Example {}
/// impl BlobProvider for Example {
///     fn get_blob(&self) -> FcResult<Blob> {
///         let example_blob = Blob::default();
///         Ok(example_blob)
///     }
/// }
/// impl Hashable for Example {
///     fn get_hash(&self) -> FcResult<String> {
///         (self as &(dyn BlobProvider)).get_hash()
///     }
/// }
/// ```
/// In the above example, "(self as &(dyn BlobProvider))" casts IndexFile
/// to a reference (since &self's already a reference) of the trait object
/// (dyn BlobProvider) we're implementing Hashable for here, in order to
/// gain access to the herein implemented method get_hash.
impl<'maybe_not_static> Hashable for dyn BlobProvider + 'maybe_not_static {
    fn get_hash(&self) -> FcResult<String> {
        Ok(blake3::hash(&self.clone_blob()?).to_hex().to_string())
    }
}