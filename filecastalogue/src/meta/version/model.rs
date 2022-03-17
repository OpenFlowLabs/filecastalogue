use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Version {
    pub index: Option<String>
}

impl Version {
    pub fn new_with_index(index: &str) -> Self {
        Self {
            index: Some(index.to_owned())
        }
    }

    pub fn new() -> Self {
        Self {
            index: None
        }
    }
}