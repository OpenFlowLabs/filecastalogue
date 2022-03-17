use std::{env::current_dir, fs::{File, create_dir_all}, io::Write,
path::{Path, PathBuf}};
use crate::error::{ErrorKind, Error, FcResult};

/// A way to join paths so that the joined element won't overwrite the base.
pub trait SafeTestPathJoin {
    fn safe_join<P: AsRef<Path>>(&self, path: P) -> FcResult<PathBuf>;
}

impl SafeTestPathJoin for Path {
    fn safe_join<P: AsRef<Path>>(&self, path: P) -> FcResult<PathBuf> {
        if path.as_ref().is_absolute() {
            Err(error!(
                ErrorKind::TestSetupSafetyCheckFailed,
                "Checking for absolute adjoining path when joining paths safely.",
                payload => format!(
                    "Path: {:?}, adjoining path: {:?}",
                    self, path.as_ref())
            ))
        }
        else {
            Ok(self.join(path))
        }
    }
}

/// Holds basic info, such as paths, describing a test Repo's layout.
/// 
/// In order to support struct literal syntax for constants, the paths
/// are &str.
/// 
/// `relative_repo_base_path` is meant to be relative to yet another
/// base path, such as a tmp dir location. The other paths are relative
/// to `relative_repo_base_Path`.
/// 
/// That "yet another" base path is supplied to our methods directly,
/// enabling our configuration to be used relative to any location, whilst
/// still retaining our ability to be used in the definition of global
/// constants.
pub(crate) struct RepoTestConf {
    pub(super) relative_repo_base_path: &'static str,
    pub(super) relative_blob_dir_path: &'static str,
    pub(super) relative_state_file_path: &'static str,
}

impl RepoTestConf {
    /// Get the absolute Repo base path for this configuration.
    /// 
    /// The `base_path` param provides the absolute location which the
    /// Repo path is relative to.
    pub(crate) fn get_repo_path(&self, base_path: &Path) -> FcResult<PathBuf>{
        Ok(PathBuf::from(base_path).safe_join(self.relative_repo_base_path)?)
    }

    pub(crate) fn get_blob_dir_path(&self, base_path: &Path)
    -> FcResult<PathBuf> {
        self.get_repo_path(base_path)?.safe_join(self.relative_blob_dir_path)
    }

    pub(crate) fn get_state_file_path(&self, base_path: &Path)
    -> FcResult<PathBuf> {
        self.get_repo_path(base_path)?.safe_join(self.relative_state_file_path)
    }
}

pub(crate) struct RepoTestSite<Base: BaseTestDir> {
    pub(super) state: &'static str,
    pub(super) base_dir: Base,
    pub(super) conf: RepoTestConf,
}

impl<Base: BaseTestDir> RepoTestSite<Base> {

    /// This creates our directory structure and state.json file.
    /// 
    /// This is basically like a ::new method, but not for an object,
    /// but the repo test site on the filesystem, which, if you so
    /// want, is our true state.
    pub(crate) fn set_up(&self) -> FcResult<PathBuf> {

        let base_path = self.base_dir.get_path()?;
        let repo_path = self.conf.get_repo_path(&base_path)?;
        
        self.base_dir.set_up()?;
        create_dir_all(self.conf.get_repo_path(&base_path)?)?;
        create_dir_all(self.conf.get_blob_dir_path(&base_path)?)?;
        self.set_up_state_file()?;
        
        Ok(repo_path)
    }

    pub(crate) fn get_state_file_path(&self) -> FcResult<PathBuf> {
        self.conf.get_state_file_path(&self.base_dir.get_path()?)
    }

    pub(crate) fn get_repo_path(&self) -> FcResult<PathBuf> {
        self.conf.get_repo_path(&self.base_dir.get_path()?)
    }

    pub(crate) fn get_blob_dir_path(&self) -> FcResult<PathBuf> {
        self.conf.get_blob_dir_path(&self.base_dir.get_path()?)
    }

    /// Get a Write for the state file for this RepoTestSite.
    /// 
    /// Calling this assumes that our .set_up method has already
    /// been called. Without that, the file might not exist and
    /// and and this will error out.
    pub(crate) fn get_state_writeable(&self) -> FcResult<File> {
        Ok(File::create(self.get_state_file_path()?)?)
    }

    pub(crate) fn get_state_readable(&self) -> FcResult<File> {
        Ok(File::open(self.get_state_file_path()?)?)
    }

    /// Sets up our state.json with the configured contents.
    /// 
    /// This is intended to be called in our .set_up method.
    pub(crate) fn set_up_state_file(&self) -> FcResult<()> {
        self.get_state_writeable()?.write_all( self.state.as_bytes())?;
        Ok(())
    }
}

/// A directory to house various volatile test directories and files.
/// 
/// Currently, the path is determined in its implementation and not
/// directly configurable.
pub(crate) struct TmpTestDir {}

pub trait BaseTestDir {
    fn get_path(&self) -> FcResult<PathBuf>;
    fn set_up(&self) -> FcResult<PathBuf>;
}

impl TmpTestDir {

    /// Gets the path to be used for the tmp test dir.
    /// 
    /// Doesn't create the dir and performs no checks whatsoever.
    /// This is intended to be used by whatever does that.
    fn get_path(&self) -> FcResult<PathBuf> {
        Ok(get_tmp_dir_path()?.safe_join("test")?)
    }

    fn set_up(&self) -> FcResult<PathBuf> {
        let path = self.get_path()?;
        create_dir_all(&path)?;
        if path.is_dir() {
            Ok(path)
        }
        else {
            Err(error!(
                ErrorKind::TestSetupSafetyCheckFailed,
                "Checking tmp test dir after creating it.",
                payload => format!("Path: {:?}", path)
            ))
        }
    }
}

impl BaseTestDir for TmpTestDir {

    /// Gets the path to be used for the tmp test dir.
    /// 
    /// Doesn't create the dir and performs no checks whatsoever.
    /// This is intended to be used by whatever does that.
    fn get_path(&self) -> FcResult<PathBuf> {
        self.get_path()
    }

    fn set_up(&self) -> FcResult<PathBuf> {
        self.set_up()
    }
}

pub(crate) fn get_tmp_dir_path() -> FcResult<PathBuf> {
    Ok(current_dir()?.safe_join(".tmp")?)
}