use std::{convert::TryFrom, fmt::Debug};
use serde::{Deserialize, Serialize};
use crate::{error::{Error, FcResult}, files::{blob::BlobProvider, tracked_ordinary_blob::TrackedOrdinaryBlobProvider}};
use super::model::Blob;

pub trait SerdeBlob: Debug {}
// pub trait RepoExportedBlobProvider: BlobProvider + SerdeBlob {}
pub trait RepoExportedOrdinaryBlobProvider: BlobProvider {}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct RepoExportedHeapOrdinaryBlobProvider {
    blob: Blob
}

impl RepoExportedHeapOrdinaryBlobProvider {

    pub fn new(blob: Blob) -> Self {
        Self {
            blob: blob
        }
    }

    pub fn from_tracked_ordinary_blob_provider(blob_provider: Box<dyn TrackedOrdinaryBlobProvider>) -> FcResult<Self> {
        Ok(Self::new(
            blob_provider.into_blob()?
        ))
    }
}

impl TryFrom<Box<dyn RepoExportedOrdinaryBlobProvider>> for Blob {
    type Error = Error;

    fn try_from(blob_provider: Box<dyn RepoExportedOrdinaryBlobProvider>) -> FcResult<Self> {
            Ok(blob_provider.into_blob()?)
    }
}

impl BlobProvider for RepoExportedHeapOrdinaryBlobProvider {

    fn clone_blob(&self) -> crate::error::FcResult<Blob> {
        Ok(self.blob.clone())
    }

    fn into_blob(self: Box<Self>) -> crate::error::FcResult<Blob> {
        Ok(self.blob)
    }
}

impl RepoExportedOrdinaryBlobProvider for RepoExportedHeapOrdinaryBlobProvider {}