use crate::error::{Error, ErrorKind};
use crate::meta::state::model::State;
use crate::meta::version::model::Version;
use crate::meta::state::error::{
    VersionEntryDoesNotExistErrorPayload,
    VersionEntryAlreadyExistsErrorPayload
};

pub(crate) const MINIMAL_STATE_VERSION_ID: usize = 0;
pub(crate) const NON_EXISTENT_VERSION_ID: usize = 1;
pub(crate) const MINIMAL_STATE_JSON: &str =
r#"{
    "versions": [
        {
            "index": "MOCKHASH"
        }
    ]
}"#;

pub(crate) const VERSION_ENTRY_ALREADY_EXISTS_ERROR_CONTEXT_DESCRIPTION: &str =
"This is a mock of the error for the case when a version entry already exists.";
pub(crate) const VERSION_ENTRY_DOES_NOT_EXIST_ERROR_DESCRIPTION: &str = 
"This is a mock of the error for the case where a version entry doesn't exist.";

pub(in crate::tests) fn create_minimal_state_struct() -> State {
    let mut mock_versions: Vec<Version> = vec!();
    mock_versions.insert(
        MINIMAL_STATE_VERSION_ID,
        Version::new_with_index("MOCKHASH")
    );
    State {
        versions: mock_versions
    }
}

pub(in crate::tests) fn create_minimal_state_VersionEntryAlreadyExistsError()
-> Error {
    error!(
        ErrorKind::VersionEntryAlreadyExists,
        VERSION_ENTRY_ALREADY_EXISTS_ERROR_CONTEXT_DESCRIPTION,
        payload => VersionEntryAlreadyExistsErrorPayload {
            version_index: MINIMAL_STATE_VERSION_ID,
            version_struct: Version::new_with_index("MOCKHASH")
        }
    )
}

pub(in crate::tests) fn create_minimal_state_VersionEntryDoesNotExistError() 
-> Error {
    error!(
        ErrorKind::VersionEntryDoesNotExist,
        VERSION_ENTRY_DOES_NOT_EXIST_ERROR_DESCRIPTION,
        payload => VersionEntryDoesNotExistErrorPayload {
                version_index: MINIMAL_STATE_VERSION_ID,
        }
    )
}