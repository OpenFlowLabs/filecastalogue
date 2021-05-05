use std::collections::HashMap;
use crate::meta::state::model::Version;
use crate::meta::state::model::State;

pub(crate) const MINIMAL_STATE_JSON: &str =
r#"{
    "versions": {
        "1": {
            "index": "MOCKHASH"
        }
    }
}"#;
pub(in crate::tests) fn create_minimal_state_struct() -> State {
    let mut mock_versions: HashMap<String, Version> = HashMap::new();
    mock_versions.insert(String::from("1"), Version {index: String::from("MOCKHASH")});
    State {
        versions: mock_versions
    }
}