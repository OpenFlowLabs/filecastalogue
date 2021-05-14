use crate::{meta::state, meta::state::{accessor::{
        Accessor,
        VersionEntryAlreadyExistsError,
        VersionEntryDoesNotExistError
    }, model::Version}, tests::fixtures::{self, models::{NON_EXISTENT_VERSION_ID, VERSION_ENTRY_DOES_NOT_EXIST_ERROR_DESCRIPTION}}};

use super::fixtures::models::MINIMAL_STATE_VERSION_ID;

// This is a proxy for "is the State struct serializing using serde_json?".
// It's a baseline check as to whether anything is working at all, really.
#[test]
fn hash_is_same_in_serialized_state() -> Result<(), serde_json::Error> {
    let state: state::model::State = serde_json::from_str(fixtures::models::MINIMAL_STATE_JSON)?;
    assert_eq!(
        state.versions["1"].index,
        fixtures::models::create_minimal_state_struct().versions["1"].index
    );
    Ok(())
}

#[test]
fn has_version_returns_true_when_state_has_version() -> () {
    let mut state = fixtures::models::create_minimal_state_struct();
    assert_eq!(state.has_version(MINIMAL_STATE_VERSION_ID), true);
}

#[test]
fn has_version_returns_false_when_state_does_not_have_version() -> () {
    let mut state = fixtures::models::create_minimal_state_struct();
    assert_eq!(state.has_version(NON_EXISTENT_VERSION_ID), false);
}

#[test]
fn get_version_returns_version() -> () {
    let mut state = fixtures::models::create_minimal_state_struct();
    let result: state::model::Version = state.get_version("1").ok().unwrap();
    assert_eq!(result, state.versions[MINIMAL_STATE_VERSION_ID]);
}

#[test]
fn get_version_returns_error_when_state_does_not_have_version() -> () {
    let mut state = fixtures::models::create_minimal_state_struct();
    let result: Result<Version, VersionEntryDoesNotExistError> = state.get_version(
        NON_EXISTENT_VERSION_ID
    );
    let is_err = result.is_err();
    assert!(is_err, format!(
        "{}. Result: {:?}",
        ".get_version() did not return an error when it should have",
        result)
    );
}

#[test]
fn put_version() -> () {
    let new_id = "2";
    let new_hash = "NEWHASH";
    let mut state = fixtures::models::create_minimal_state_struct();
    assert_ne!(state.has_version(new_id), true,
        "Preparation failed: State shouldn't have the version ({}) we're about to insert yet.",
        new_id
    );
    state.put_version(new_id, new_hash);
    assert_eq!(state.has_version(new_id), true);
}

#[test]
fn add_version() -> () {
    let new_id = "2";
    let new_hash = "NEWHASH";
    let mut state = fixtures::models::create_minimal_state_struct();
    assert_ne!(state.has_version(new_id), true,
        "Preparation failed: State shouldn't have the version ({}) we're about to insert yet.",
        new_id
    );
    let result = state.add_version(new_id, new_hash);
    assert_ne!(result.is_err(), true, 
        "Preparation failed: .has_version() shouldn't return an error here. Version ID: {}",
        new_id
    );
    assert_eq!(state.has_version(new_id), true);
}

#[test]
fn del_version() -> () {
    let mut state = fixtures::models::create_minimal_state_struct();
    assert_eq!(state.has_version(MINIMAL_STATE_VERSION_ID), true,
        "Preparation failed: .has_version() should return true here for ID: {}",
        MINIMAL_STATE_VERSION_ID
    );
    state.del_version(MINIMAL_STATE_VERSION_ID);
    assert_ne!(state.has_version(MINIMAL_STATE_VERSION_ID), true);
}