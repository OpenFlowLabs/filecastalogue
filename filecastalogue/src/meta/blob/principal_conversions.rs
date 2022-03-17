use std::{convert::TryFrom, io::Read};
use crate::error::Error;
use super::model::Blob;

/// Principal conversions between Blob and various other forms.
/// 
/// By convention, only these conversions should be used in order
/// to obtain the associated forms, in order to prevent code sprawl
/// of the processes used for these conversions. That way, there's
/// only one way to, say, obtain a Blob from a Read, which helps
/// with the maintainability of code that depends on that.

impl TryFrom<&mut (dyn Read)> for Blob {
    type Error = Error;

    /// Principal conversion from Read to Blob.
    /// 
    /// This is the one way which should be used to obtain a
    /// Blob directly from a Read.
    fn try_from(readable: &mut (dyn Read)) -> Result<Self, Self::Error> {
        let mut blob: Blob = Blob::default();
        readable.read_to_end(&mut blob)?;
        Ok(blob)
    }
}

impl From<Vec<u8>> for Blob {

    /// Principal conversion from Vec<u8> to Blob.
    /// 
    /// This is the one way which should be used to obtain a
    /// Blob directly from a Vec<u8>.
    /// 
    /// This does nothing else but wrap the specified Vec<u8>
    /// blob in a Blob.
    fn from(blob_vec: Vec<u8>) -> Self {
        Blob::from_vec(blob_vec)
    }
}

impl From<Blob> for Vec<u8> {

    /// Principal conversion from Blob to Vec<u8> .
    /// 
    /// This is the one way which should be used to obtain a
    /// Vec<u8> directly from a Blob.
    /// 
    /// This simply returns the Blob's inner field.
    fn from(blob: Blob) -> Self {
        blob.into_vec()
    }
}