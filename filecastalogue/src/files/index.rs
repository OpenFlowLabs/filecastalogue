use crate::{error::FcResult, meta::index::model::Index};

pub mod drivers {
    pub mod local;
}

/**
Somehow provides index data and a means to set the index data
it provides, at least for as long as it's alive. Unless specifically
pointed out, the methods on the provier aren't concerned with
with where the data is coming from, where it goes, whether it's
persistent or anything like that.

Other means will have to provide for that (e.g. another trait).
*/
pub trait IndexProvider {
    fn get_index(self: &mut Self) -> FcResult<&crate::meta::index::model::Index>;
    fn set_index(self: &mut Self, index: &dyn AsRef<Index>)
    -> FcResult<()>;
}