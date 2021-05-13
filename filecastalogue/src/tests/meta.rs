use crate::{
    meta::state,
    // meta::index,
    tests::fixtures
};

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