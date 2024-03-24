use std::ffi::OsString;
use crate::error::FcTestResult;
use crate::meta::file_aspects::aspects::non_existing::TrackableNonExistingAspects;
use crate::meta::repo_exported_file_list::model::RepoExportedVecFileList;
// Instead of importing all fixtures directly, we prefix
// calls to fixtures with `test_fixtures`, to make things clearer.
use crate::tests::test_fixtures;
// For as long as constants aren't used regularly in the code being
// tested, dropping the "prefix" idea for them is worth the shorter
// statements. Refactor once this gets confusing for a particular
// category.
use crate::tests::test_fixtures::repo::NON_EXISTING_VERSION_INDEX;
use crate::tests::test_ids::TestIDs;

#[test]
fn has_version_returns_false_when_repo_does_not_have_version() -> FcTestResult<()> {
    let mut repo = test_fixtures::repo::create_minimal_repo_struct(
        TestIDs::RepoHasVersionReturnsFalseWhenRepoDoesNotHaveVersion.as_str()
    )?;
    assert_eq!(repo.has_version(NON_EXISTING_VERSION_INDEX)?, false);
    Ok(()).into()
}

/* TODO: Using `::create_minimal_repo_struct` and then `.get_blob_dir_path` is
 *  a test maintenance hazard. Perhaps `::create_minimal_repo_struct` should
 *  return some additional info (e.g. a struct that has the repo and some meta
 *  data), or `MINIMAL_REPO_SITE` should be what houses
 *  `create_minimal_repo_struct` (that actually sounds preferable).
*/
/// Happy path testing of `Repo::add_version`.
#[test]
fn add_version_succeeds() -> FcTestResult<()> {
    let mut repo = test_fixtures::repo::create_minimal_repo_struct(
        TestIDs::RepoAddVersionSucceeds.as_str()
    )?;
    let new_version_index = repo.add_version()?;
    
    assert_eq!(repo.has_version(new_version_index)?, true);
    Ok(()).into()
}

/// Comprehensive happy path testing of `Repo::track_non_existing`.
#[test]
fn track_non_existing_succeeds() -> FcTestResult<()> {
    let file_path = OsString::from("/this/does/not/exist");
    let trackable_aspects = TrackableNonExistingAspects::new();
    
    let mut repo = test_fixtures::repo::create_minimal_repo_struct(
        TestIDs::RepoTrackNonExistingSucceeds.as_str()
    )?;

    let maybe_new_version_index = repo.add_version();
    assert_eq!(maybe_new_version_index.is_err(), false,
        "Running `repo.add_version()` returned an error: {:?}", maybe_new_version_index.unwrap_err());
    let new_version_index = maybe_new_version_index.unwrap();

    repo.track_non_existing(new_version_index, file_path.clone(), trackable_aspects)?;

    let mut file_list = RepoExportedVecFileList::new();
    repo.get_files(new_version_index, &mut file_list)?;

    assert!(file_list.into_iter().any(
        |tracked_file| -> bool { tracked_file.get_path() == file_path }
    ));

    Ok(()).into()
}