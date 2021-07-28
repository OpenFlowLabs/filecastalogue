use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Version {
    pub index: String
}

impl Version {
    pub fn new(index: &str) -> Self {
        Self {
            index: index.to_owned()
        }
    }
}