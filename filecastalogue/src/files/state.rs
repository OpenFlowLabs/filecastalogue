pub mod drivers {
    pub mod local;
}

pub trait StateProvider {
    fn get_state(self: &mut Self) -> crate::meta::state::model::State;
}