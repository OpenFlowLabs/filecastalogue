use crate::{error::FcResult, meta::state::model::State};

pub mod drivers {
    pub mod local;
}

pub trait StateProvider {
    fn get_state(self: &mut Self) -> FcResult<&State>;
}