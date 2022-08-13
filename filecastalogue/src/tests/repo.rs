use std::ffi::OsString;
use crate::error::{ErrorPathBuf, FcTestResult};
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
use crate::tests::test_ids::TestIDs;
use std::backtrace::Backtrace;

#[test]
fn has_version_returns_false_when_repo_does_not_have_version() -> FcTestResult<()> {
    let mut repo = test_fixtures::repo::create_minimal_repo_struct(
        TestIDs::RepoHasVersionReturnsFalseWhenRepoDoesNotHaveVersion.as_str()
    )?;
    assert_eq!(repo.has_version(NON_EXISTENT_VERSION_ID)?, false);
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
    let version_id = "added_version";
    let add_version_result = repo.add_version(version_id);    
    assert_eq!(add_version_result.is_err(), false, "{}, {}. {}, {:?}.",
        "Error when trying to add version: ", add_version_result.err().unwrap(),
        // Abusing ErrorPathBuf here to benefit from its detection and warning
        // system when it encounters paths which aren't convertible by `.as_str`,
        // and instead have to be converted lossily.
        // NOTE: I feel this has not been sufficiently tested, so, beware.
        "Blob dir path: ", ErrorPathBuf::from(MINIMAL_REPO_SITE.get_blob_dir_path(
            TestIDs::RepoAddVersionSucceeds.as_str()
        )?)
    );
    
    assert_eq!(repo.has_version(version_id)?, true);
    Ok(()).into()
}

/// Comprehensive happy path testing of `Repo::track_non_existing`.
#[test]
fn track_non_existing_succeeds() -> FcTestResult<()> {
    let version_id = "version_with_non_existing_file";
    let file_path = OsString::from("/this/does/not/exist");
    let trackable_aspects = TrackableNonExistingAspects::new();
    
    let mut repo = test_fixtures::repo::create_minimal_repo_struct(
        TestIDs::RepoTrackNonExistingSucceeds.as_str()
    )?;
    repo.add_version(version_id)?;
    // TODO: Fix Some(Serde(Error("key must be a string"...
    repo.track_non_existing(version_id, file_path.clone(), trackable_aspects)?;

    let mut file_list = RepoExportedVecFileList::new();
    repo.get_files(version_id, &mut file_list)?;

    // TODO: Implement checking `file_list` to see whether we're now tracking
    // the non-existing file.
    assert!(file_list.into_iter().any(
        |tracked_file| -> bool { tracked_file.get_path() == file_path }
    ));

    Ok(()).into()
}