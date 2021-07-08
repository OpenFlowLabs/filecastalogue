use std::{ffi::{OsStr, OsString}, path::{Path, PathBuf}};

pub struct HashFileName {
    string: OsString
}

impl From<&str> for HashFileName {
    fn from(input_str: &str) -> Self {
        Self {
            string: OsString::from(input_str)
        }
    }
}

impl From<String> for HashFileName {
    fn from(input_string: String) -> Self {
        Self {
            string: OsString::from(input_string)
        }
    }
}

impl From<&OsStr> for HashFileName {
    fn from(input_os_str: &OsStr) -> Self {
        Self {
            string: OsString::from(input_os_str)
        }
    }
}

impl From<HashFileName> for &OsStr {
    fn from(input_hash_file_name: HashFileName) -> Self {
        &input_hash_file_name.string
    }
}

// impl From<&HashFileName> for &OsStr {
//     fn from(input_hash_file_name: &HashFileName) -> Self {
//         &input_hash_file_name.string
//     }
// }

impl From<OsString> for HashFileName {
    fn from(input_os_string: OsString) -> Self {
        Self {
            string: input_os_string
        }
    }
}

impl From<PathBuf> for HashFileName {
    fn from(input_path_buf: PathBuf) -> Self {
        Self {
            string: OsString::from(input_path_buf)
        }
    }
}

impl From<&Path> for HashFileName {
    fn from(input_path: &Path) -> Self {
        Self {
            string: OsString::from(input_path)
        }
    }
}

// impl TryInto<&OsStr> for HashFileName {
//     type Error = Error;

//     fn try_into(self) -> Result<&'static OsStr, Self::Error> {
//         Ok(&self.string)
//     }
// }