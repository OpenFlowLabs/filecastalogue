use crate::{error::FcResult, tests::test_fixtures::repo::NON_EXISTENT_VERSION_ID,
tests::test_fixtures};

#[test]
fn has_version_returns_false_when_repo_does_not_have_version() -> FcResult<()> {
    let mut repo = test_fixtures::repo::create_minimal_repo_struct()?;
    assert_eq!(repo.has_version(NON_EXISTENT_VERSION_ID)?, false);
    Ok(())
}