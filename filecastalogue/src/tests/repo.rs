use crate::{
    error::FcResult,
    meta::{
        version::model::Version
    },
    tests::test_fixtures::repo::NON_EXISTENT_VERSION_ID,
    tests::test_fixtures
};

#[test]
fn has_version_returns_false_when_repo_does_not_have_version() -> FcResult<()> {
    let mut repo = test_fixtures::repo::create_minimal_repo_struct()?;
    assert_eq!(repo.has_version(NON_EXISTENT_VERSION_ID)?, false);
    Ok(())
}

/// Happy path testing of `Repo::add_version`.
#[test]
fn add_version_succeeds() -> FcResult<()> {
    let version_id = "added_version";
    let version = Version::new();
    
    let mut repo = test_fixtures::repo::create_minimal_repo_struct()?;
    repo.add_version(version_id, version)?;
    
    assert_eq!(repo.has_version(version_id)?, true);
    Ok(())
}
