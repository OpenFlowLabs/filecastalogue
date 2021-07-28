use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use super::super::version::model::Version;

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct State {
    pub versions: HashMap<String, Version>
}