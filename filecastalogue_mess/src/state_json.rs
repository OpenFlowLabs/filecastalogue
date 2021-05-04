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