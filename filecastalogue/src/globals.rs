/// Crate wide global values.
/// 
/// Each global is supposed to be a single sources of truth for what it
/// represents. Thus for a global called "STATE_FILE_NAME" there should be
/// no competing source of truth for the name of the state file. If
/// there is, the global's name should communicate a narrowed scope of
/// what the global is supposed to be a source of truth of, e.g. it
/// should be named DEFAULT_STATE_FILE_NAME if that's what it *really*
/// is (or might have become due to the library evolving that way), as
/// then it'd be the single source of truth for the *default* state file
/// name, not the state file name overall, and the name should reflect
/// that.

/// The state file's name in fileoid repos.
pub(crate) const STATE_FILE_NAME: &str = "state.json";
/// The name of the directory where the blobs are in fileoid repos.
pub(crate) const BLOBS_DIR_NAME: &str = "blobs";