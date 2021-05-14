use std::collections::HashMap;
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Version {
    pub index: String
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct State {
    pub versions: HashMap<String, Version>
}