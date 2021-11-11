use std::ffi::OsString;
use crate::error::ErrorKind;
use crate::error::FcResult;
use crate::meta::file_aspects::aspects::non_existing::TrackableNonExistingAspects;
use crate::meta::repo_exported_file_list::model::RepoExportedVecFileList;
use crate::tests::TEST_CONF::MINIMAL_REPO_SITE;
// Instead of importing all fixtures directly, we prefix
// calls to fixtures with `test_fixtures`, to make things clearer.
use crate::tests::test_fixtures;
// For as long as constants aren't used regularly in the code being
// tested, dropping the "prefix" idea for them is worth the shorter
// statements.
use crate::tests::test_fixtures::repo::NON_EXISTENT_VERSION_ID;

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
    
    let mut repo = test_fixtures::repo::create_minimal_repo_struct()?;
    let add_version_result = repo.add_version(version_id);    
    assert_eq!(add_version_result.is_err(), false, "{}, {}. {}, {:?}.",
        "Error when trying to add version: ", add_version_result.err().unwrap(),
        "Directory path: ", MINIMAL_REPO_SITE.get_repo_path()?
    );
    
    assert_eq!(repo.has_version(version_id)?, true);
    Ok(())
}

/// Comprehensive happy path testing of `Repo::track_non_existing`.
#[test]
fn track_non_existing_succeeds() -> FcResult<()> {
    let version_id = "version_with_non_existing_file";
    let file_path = OsString::from("/this/does/not/exist");
    let trackable_aspects = TrackableNonExistingAspects::new();
    
    let mut repo = test_fixtures::repo::create_minimal_repo_struct()?;
    repo.add_version(version_id)?;
    repo.track_non_existing(version_id, file_path, trackable_aspects)?;

    let mut file_list = RepoExportedVecFileList::new();
    repo.get_files(version_id, &mut file_list)?;

    // TODO: Implement checking `file_list` to see whether we're now tracking
    // the non-existing file.
    todo!();

    Ok(())
}