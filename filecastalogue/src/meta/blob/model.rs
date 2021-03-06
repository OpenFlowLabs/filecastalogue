use std::ops::{Deref, DerefMut};
use serde::{Deserialize, Serialize};

/// Represents the blob portion of an ordinary file.
/// 
/// This is basically a thin wrapper around Vec<u8> with Deref and DerefMut,
/// as well as TryFrom based principal conversions as applicable. It seeks
/// to provide a default goto-point for single sources of process for blob
/// related functionality.
/// 
/// A Default implementation based on vec!() is also provided.
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Blob {
    inner: Vec<u8>
}

impl Blob {
    
    pub fn from_vec(blob: Vec<u8>) -> Self {
        Self {
            inner: blob
        }
    }

    pub fn into_vec(self) -> Vec<u8> {
        self.inner
    }

}

impl Default for Blob {
    /// Initializes with vec!().
    fn default() -> Self {
        Blob {
            inner: vec!()
        }
    }
}

impl Clone for Blob {
    fn clone(&self) -> Self {
        Blob::from_vec(Vec::clone(&self.inner))
    }
}

impl Deref for Blob {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for Blob {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}