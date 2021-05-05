use std::collections::HashMap;
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct Version {
    pub index: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct State {
    pub versions: HashMap<String, Version>
}
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