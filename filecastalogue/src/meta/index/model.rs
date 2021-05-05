use serde::{Serialize, Deserialize};
use std::{collections::HashMap};

#[derive(Serialize, Deserialize, Debug)]
pub struct Attributes {
    pub posix_user: String,
    pub posix_group: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NonExistingFileKind {}

#[derive(Serialize, Deserialize, Debug)]
pub struct DirectoryFileKind {
    pub attributes: Attributes
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FileFileKind {
    pub hash: String,
    pub attributes: Attributes
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SymlinkFileKind {
    pub linked_to: String
}

// struct HardlinkFileKind {

// }

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    tag = "kind",
    rename_all(
        serialize = "snake_case",
        deserialize = "snake_case"
    )
)]
pub enum FileAspects {
    NonExisting(NonExistingFileKind),
    Directory(DirectoryFileKind),
    File(FileFileKind),
    Symlink(SymlinkFileKind)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Index {
    pub files: HashMap<String, FileAspects>
}