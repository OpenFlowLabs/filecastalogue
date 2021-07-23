use crate::error::FcResult;

pub trait Hashable {
    fn get_hash(&self) -> FcResult<String>;
}