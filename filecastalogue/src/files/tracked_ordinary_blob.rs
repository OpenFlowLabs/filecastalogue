use std::{convert::{TryInto}, io::{Read, Write}};
use crate::{error::{FcResult}, meta::blob::model::Blob};
use super::{RepoFile, blob::BlobProvider, hashable::Hashable};

/* Notes:
        There are two reasons why this exists:
        - Consistency with the "Provider" pattern of other RepoFiles.
            The rest of the world doesn't care that Tracked files are special
            as in that their canonical content is simply their binary blob.
        - It decouples the concern of "Tracked file provides things" from
            "blob files have blobs".
*/
/// Intended for access to the actual binary blob of a Tracked file.
pub trait TrackedOrdinaryBlobProvider: BlobProvider {}

/// This is the codified representation of a Tracked blob file, a file
/// that's stored in the Repo with the purpose of holding the binary blob
/// of a file we're tracking.
pub trait RepoTrackedOrdinaryBlobFile: RepoFile + TrackedOrdinaryBlobProvider
+ BlobProvider + Hashable {
    fn as_tracked_ordinary_blob_provider_ref(&self)
    -> &dyn TrackedOrdinaryBlobProvider;

    fn as_tracked_ordinary_blob_provider_box(self: Box<Self>)
    -> Box<dyn TrackedOrdinaryBlobProvider>;
}

pub struct TrackedOrdinaryBlobFile {
    pub blob: Blob
}

impl TrackedOrdinaryBlobFile {
    pub fn from_existing(readable: &mut (dyn Read)) -> FcResult<Self> {
        Ok(Self {
            blob: readable.try_into()?
        })
    }
}

impl RepoFile for TrackedOrdinaryBlobFile {
    fn load(self: &mut Self, readable: &mut (dyn Read)) -> FcResult<()> {
        self.blob = readable.try_into()?;
        Ok(())
    }

    fn save(self: &mut Self, writeable: &mut (dyn Write))-> FcResult<()> {
        writeable.write(&self.blob)?;
        Ok(())
    }
}

impl BlobProvider for TrackedOrdinaryBlobFile {

    fn clone_blob(&self) -> FcResult<Blob> {
        Ok(self.blob.clone())
    }

    fn into_blob(self: Box<Self>) -> FcResult<Blob> {
        Ok(self.blob)
    }

    
}

impl Hashable for TrackedOrdinaryBlobFile {
    fn get_hash(&self) -> FcResult<String> {
        (self as &(dyn BlobProvider)).get_hash()
    }
}

impl TrackedOrdinaryBlobProvider for TrackedOrdinaryBlobFile {}

impl RepoTrackedOrdinaryBlobFile for TrackedOrdinaryBlobFile {
    // TODO [typing]: This is a cheap solution to upcasting.
    //  Find a better way. ^^"
    /// Upcasts from the `RepoTrackedOrdinaryBlobFile` trait
    /// into `TrackedOrdinaryBlobProvider`, which is a super-trait 
    /// of the former.
    fn as_tracked_ordinary_blob_provider_ref(&self)
    -> &dyn TrackedOrdinaryBlobProvider {
        self
    }

    fn as_tracked_ordinary_blob_provider_box(self: Box<Self>)
    -> Box<dyn TrackedOrdinaryBlobProvider> {
        self
    }
}
