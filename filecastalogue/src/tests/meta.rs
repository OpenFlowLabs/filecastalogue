use crate::{error::FcResult, meta::state, meta::{state::accessor::StateAccessor, version::model::Version}, meta::version, tests::test_fixtures::{
        self,
        models::{NON_EXISTENT_VERSION_ID}
    }};
use super::test_fixtures::models::MINIMAL_STATE_VERSION_ID;

// This is a proxy for "is the State struct serializing using serde_json?".
// It's a baseline check as to whether anything is working at all, really.
#[test]
fn hash_is_same_in_serialized_state() -> FcResult<()> {
    let state: state::model::State = serde_json::from_str(test_fixtures::models::MINIMAL_STATE_JSON)?;
    assert_eq!(
        state.versions["1"].index,
        test_fixtures::models::create_minimal_state_struct().versions["1"].index
    );
    Ok(())
}

#[test]
fn has_version_returns_true_when_state_has_version() -> () {
    let mut state = test_fixtures::models::create_minimal_state_struct();
    assert_eq!(state.has_version(MINIMAL_STATE_VERSION_ID), true);
}

#[test]
fn has_version_returns_false_when_state_does_not_have_version() -> () {
    let mut state = test_fixtures::models::create_minimal_state_struct();
    assert_eq!(state.has_version(NON_EXISTENT_VERSION_ID), false);
}

#[test]
fn get_version_returns_version() -> () {
    let mut state = test_fixtures::models::create_minimal_state_struct();
    let result: version::model::Version = state.get_version("1").ok().unwrap();
    assert_eq!(result, state.versions[MINIMAL_STATE_VERSION_ID]);
}

#[test]
fn get_version_returns_error_when_state_does_not_have_version() -> () {
    let mut state = test_fixtures::models::create_minimal_state_struct();
    let result = state.get_version(
        NON_EXISTENT_VERSION_ID
    );
    let is_err = result.is_err();
    assert!(is_err,
        "{}. Result: {:?}",
        ".get_version() did not return an error when it should have",
        result
    );
}

#[test]
fn put_version() -> () {
    let new_id = "2";
    let new_hash = "NEWHASH";
    let mut state = test_fixtures::models::create_minimal_state_struct();
    assert_ne!(state.has_version(new_id), true,
        "Preparation failed: State shouldn't have the version ({}) we're about to insert yet.",
        new_id
    );
    state.put_version(new_id, Version::new_with_index(new_hash));
    assert_eq!(state.has_version(new_id), true);
}

#[test]
fn add_version() -> () {
    let new_id = "2";
    let new_hash = "NEWHASH";
    let mut state = test_fixtures::models::create_minimal_state_struct();
    assert_ne!(state.has_version(new_id), true,
        "Preparation failed: State shouldn't have the version ({}) we're about to insert yet.",
        new_id
    );
    let result = state.add_version(
        new_id, Version::new_with_index( new_hash)
    );
    assert_ne!(result.is_err(), true, 
        "Preparation failed: .has_version() shouldn't return an error here. Version ID: {}",
        new_id
    );
    assert_eq!(state.has_version(new_id), true);
}

#[test]
fn del_version() -> () {
    let mut state = test_fixtures::models::create_minimal_state_struct();
    assert_eq!(state.has_version(MINIMAL_STATE_VERSION_ID), true,
        "Preparation failed: .has_version() should return true here for ID: {}",
        MINIMAL_STATE_VERSION_ID
    );
    let result = state.del_version(
        MINIMAL_STATE_VERSION_ID
    );
    assert_ne!(result.is_err(), true,
    "Preparation failed: .del_version() shouldn't return an error here. Version ID: {}",
    MINIMAL_STATE_VERSION_ID
);
    assert_ne!(state.has_version(MINIMAL_STATE_VERSION_ID), true);
}