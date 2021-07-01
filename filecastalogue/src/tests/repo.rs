use crate::{error::FcResult, tests::fixtures::repo::NON_EXISTENT_VERSION_ID,
tests::fixtures};

#[test]
fn has_version_returns_false_when_repo_does_not_have_version() -> FcResult<()> {
    let mut repo = fixtures::repo::create_minimal_repo_struct()?;
    assert_eq!(repo.has_version(NON_EXISTENT_VERSION_ID), false);
    Ok(())
}