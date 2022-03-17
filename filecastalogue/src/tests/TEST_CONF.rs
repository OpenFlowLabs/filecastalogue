use super::{test_fixtures::models::MINIMAL_STATE_JSON, test_utils::{RepoTestConf, RepoTestSite, TmpTestDir}};

pub(crate) const MINIMAL_REPO_SITE: RepoTestSite<TmpTestDir> = RepoTestSite {
    state: MINIMAL_STATE_JSON,
    base_dir: TmpTestDir {},
    conf: RepoTestConf {
        relative_repo_base_path: "minimal_repo",
        relative_blob_dir_path: "blobs",
        relative_state_file_path: "state.json"
    },
};