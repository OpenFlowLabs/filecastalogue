use std::{convert::{TryFrom, TryInto}, io::Read};
use crate::{error::{Error}};
use super::model::Index;

impl TryFrom<&mut (dyn Read)> for Index {
    type Error = Error;

    fn try_from(readable: &mut (dyn Read)) -> Result<Self, Self::Error> {
        let mut blob= vec!();
        readable.read_to_end(&mut blob)?;
        let index = blob.try_into()?;
        Ok(index)
    }
}

impl TryFrom<Vec<u8>> for Index {
    type Error = Error;

    fn try_from(blob: Vec<u8>) -> Result<Self, Self::Error> {
        Ok(serde_json::from_slice(&blob)?)
    }
}

impl TryFrom<Index> for Vec<u8> {
    type Error = Error;

    fn try_from(index: Index) -> Result<Self, Self::Error> {
        Ok(serde_json::to_vec_pretty(&index)?)
    }
}