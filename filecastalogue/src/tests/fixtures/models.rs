use std::collections::HashMap;
use crate::meta::state::model::{State, Version};
use crate::meta::state::accessor::{
    VersionEntryDoesNotExistError,
    VersionEntryAlreadyExistsError
};

pub(crate) const NON_EXISTENT_VERSION_ID: &str = "0";
pub(crate) const MINIMAL_STATE_VERSION_ID: &str = "1";
pub(crate) const MINIMAL_STATE_JSON: &str =
r#"{
    "versions": {
        "1": {
            "index": "MOCKHASH"
        }
    }
}"#;
pub(crate) const VERSION_ENTRY_ALREADY_EXISTS_ERROR_CONTEXT_DESCRIPTION: &str =
"This is a mock of the error for the case when a version entry already exists.";

pub(crate) const VERSION_ENTRY_DOES_NOT_EXIST_ERROR_DESCRIPTION: &str = 
"This is a mock of the error for the case where a version entry doesn't exist.";

pub(in crate::tests) fn create_minimal_state_struct() -> State {
    let mut mock_versions: HashMap<String, Version> = HashMap::new();
    mock_versions.insert(
        String::from(MINIMAL_STATE_VERSION_ID),
        Version {
            index: String::from("MOCKHASH")
        }
    );
    State {
        versions: mock_versions
    }
}

pub(in crate::tests) fn create_minimal_state_VersionEntryAlreadyExistsError()
-> VersionEntryAlreadyExistsError {
    VersionEntryAlreadyExistsError {
        version_id: String::from(MINIMAL_STATE_VERSION_ID),
        version_struct: Version {
            index: String::from("MOCKHASH")
        },
        context_description: String::from(
            VERSION_ENTRY_ALREADY_EXISTS_ERROR_CONTEXT_DESCRIPTION
        )
    }
}

pub(in crate::tests) fn create_minimal_state_VersionEntryDoesNotExistError() 
-> VersionEntryDoesNotExistError {
    VersionEntryDoesNotExistError {
        version_id: String::from(MINIMAL_STATE_VERSION_ID),
        context_description: String::from(
            VERSION_ENTRY_DOES_NOT_EXIST_ERROR_DESCRIPTION
        )
    }
}