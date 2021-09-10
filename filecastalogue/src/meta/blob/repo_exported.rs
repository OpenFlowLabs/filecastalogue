use std::fmt::Debug;
use serde::{Deserialize, Serialize};
use super::model::Blob;

pub trait RepoExportedBlob: Debug {}

pub trait SerdeBlob {}

impl PartialEq for Box<dyn RepoExportedBlob> {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

impl PartialEq<&Self> for Box<dyn RepoExportedBlob> {
    fn eq(&self, other: &&Self) -> bool {
        todo!()
    }
}

impl Eq for Box<dyn RepoExportedBlob> {}

impl Clone for Box<dyn RepoExportedBlob> {
    fn clone(&self) -> Self {
        // Self(self.0.clone(), self.1.clone())
        todo!()
    }
}

impl Serialize for Box<dyn RepoExportedBlob> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        todo!()
    }
}

impl<'de> Deserialize<'de> for Box<dyn RepoExportedBlob> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
        todo!()
    }
}

impl RepoExportedBlob for Blob {}