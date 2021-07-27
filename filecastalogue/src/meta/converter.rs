use std::{convert::TryFrom, io::Read};
use crate::error::FcResult;

/// Single source of process conversions for State.
/// 
/// Here, we define conversion methods to be used when converting to and
/// from State (e.g. from/to a blob).
pub trait Converter<Model> {

    /// Principal method of how to obtain a deserialized State from a blob.
    fn from_blob(blob: Vec<u8>) -> FcResult<Model>;

    /// Principal method of how to obtain a deserialized State from Read.
    fn from_read(readable: &mut (dyn Read)) -> FcResult<Model>;

    /// Principal method to get a serialized State in blob form.
    fn to_blob(&self) -> FcResult<Vec<u8>>;
}