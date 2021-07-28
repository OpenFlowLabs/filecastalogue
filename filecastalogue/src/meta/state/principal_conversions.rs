use std::{convert::{TryFrom, TryInto}, io::Read};
use crate::{error::{Error}};
use super::model::State;

/// Principal conversions between State and various other forms.
/// 
/// By convention, only these conversions should be used in order
/// to obtain the associated forms, in order to prevent code sprawl
/// of the processes used for these conversions. That way, there's
/// only one way to, say, obtain a Blob from State, which helps
/// with the maintainability of code that depends on that.

impl TryFrom<&mut (dyn Read)> for State {
    type Error = Error;

    /// Principal conversion from Read to State.
    /// 
    /// This is the one way which should be used to obtain the
    /// State in deserialized form directly from a Read providing a
    /// Blob. This wraps around the principal conversion of
    /// Blob to State.
    /// 
    /// The Read must produce a serde_json deserializable Blob
    /// or this will fail.
    fn try_from(readable: &mut (dyn Read)) -> Result<Self, Self::Error> {
        let mut blob= vec!();
        readable.read_to_end(&mut blob)?;
        let index = blob.try_into()?;
        Ok(index)
    }
}

impl TryFrom<Vec<u8>> for State {
    type Error = Error;

    /// Principal conversion from Blob to State.
    /// 
    /// This is the one way which should be used to obtain the State in
    /// deserialized form from a Blob.
    /// 
    /// The Blob must be serde_json deserializable or this will fail.
    fn try_from(blob: Vec<u8>) -> Result<Self, Self::Error> {
        Ok(serde_json::from_slice(&blob)?)
    }
}

impl TryFrom<State> for Vec<u8> {
    type Error = Error;

    /// Principal conversion from State to Blob.
    /// 
    /// This is the one way which should be used to obtain a Blob from
    /// the deserialized form of the State.
    fn try_from(index: State) -> Result<Self, Self::Error> {
        Ok(serde_json::to_vec_pretty(&index)?)
    }
}