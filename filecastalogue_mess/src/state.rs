
use crate::state_json::{State, Version};

pub trait Editable {
    fn put_version(self: &mut Self, id: String, index: String) -> &mut Self;
}

impl Editable for State {
    fn put_version(&mut self, id: String, index: String) -> &mut Self {
        self.versions.insert(id,Version {
            index: String::from("TODO")
        });
        self
    }
}