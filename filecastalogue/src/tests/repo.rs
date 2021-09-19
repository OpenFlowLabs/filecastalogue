use std::ffi::OsString;

use crate::{
    error::FcResult,
    meta::{
        file_aspects::aspects::non_existing::TrackableNonExistingAspects,
        repo_exported_file_list::model::RepoExportedVecFileList,
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

/// Comprehensive happy path testing of `Repo::track_non_existing`.
#[test]
fn track_non_existing_succeeds() -> FcResult<()> {
    let version_id = "version_with_non_existing_file";
    let version = Version::new();
    let file_path = OsString::from("/this/does/not/exist");
    let trackable_aspects = TrackableNonExistingAspects::new();
    
    let mut repo = test_fixtures::repo::create_minimal_repo_struct()?;
    repo.add_version(version_id, version)?;
    repo.track_non_existing(version_id, file_path, trackable_aspects)?;

    let mut file_list = RepoExportedVecFileList::new();
    repo.get_files(version_id, &mut file_list)?;

    // TODO: Implement checking `file_list` to see whether we're now tracking
    // the non-existing file.
    todo!();

    Ok(())
}