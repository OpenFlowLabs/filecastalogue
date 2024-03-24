//! This should be used by tests which are reliant on global resources,
//! whereas namespacing the resources between tests is required and
//! simply using random identifiers isn't enough (e.g. for post-test analysis).
//! 
//! It's probably best to stick to the test's function name as closely as
//! possible, and prefix the ID with the test module name.

use std::fmt::Display;

#[derive(Debug)]
pub enum TestIDs {
    RepoHasVersionReturnsFalseWhenRepoDoesNotHaveVersion,
    RepoAddVersionSucceeds,
    RepoTrackNonExistingSucceeds,
    RepoTrackDirectorySucceeds
}

impl TestIDs {
    pub(crate) fn as_str(&self) -> &'static str {
        match *self {
            TestIDs::RepoHasVersionReturnsFalseWhenRepoDoesNotHaveVersion
                => "repo_has_version_returns_false_when_repo_does_not_have_version",
            TestIDs::RepoAddVersionSucceeds => "repo_add_version_succeeds",
            TestIDs::RepoTrackNonExistingSucceeds => "repo_track_non_existing_succeeds",
            TestIDs::RepoTrackDirectorySucceeds => "repo_track_directory_succeeds"
        }
    }
}

impl From<TestIDs> for &str {
    fn from(test_ids: TestIDs) -> Self {
        test_ids.as_str()
    }
}

impl Display for TestIDs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.as_str()
        )
    }
}