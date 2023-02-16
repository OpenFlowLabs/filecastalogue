use serde::{Serialize, Deserialize};
use super::super::version::model::Version;

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct State {
    pub versions: Vec<Version>
}

impl State {
    pub fn new() -> Self {
        Self {
            versions: vec!(),
        }
    }
}