use std::io::Read;
use crate::{error::FcResult, meta::converter::Converter};
use super::model::State;

impl Converter<State> for State {

    fn from_blob(blob: Vec<u8>) -> FcResult<State> {
        Ok(serde_json::from_slice(&blob)?)
    }

    fn from_read(readable: &mut (dyn Read)) -> FcResult<State> {
        let mut serialized= vec!();
        readable.read_to_end(&mut serialized)?;
        Ok(Self::from_blob(serialized)?)
    }

    fn to_blob(&self) -> FcResult<Vec<u8>> {
        Ok(serde_json::to_vec_pretty(self)?)
    }
}