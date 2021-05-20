use std::{ffi::{OsStr, OsString}, fmt};
use crate::meta::index::model::{Index};

use super::model::FileAspects;

pub struct FileAlreadyTrackedError {
    pub path: OsString,
    pub index_struct: Index,
    pub context_description: String,
}

impl fmt::Debug for FileAlreadyTrackedError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Error: The file at path \"{:?}\" {} {:?}. Context: {}",
            self.path,
            "is already tracked, with the following entry:",
            self.index_struct,
            self.context_description
        )
    }
}

impl fmt::Display for FileAlreadyTrackedError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Error: The file at path \"{:?}\" is already tracked.",
            self.path,
        )
    }
}

pub struct UntrackedFileError {
    pub path: OsString,
    pub context_description: String
}

impl fmt::Debug for UntrackedFileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Error: There is no file tracked for path \"{:?}\". Context: {}",
            self.path,
            self.context_description
        )
    }
}

impl fmt::Display for UntrackedFileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Error: There is no file tracked for path \"{:?}\".",
            self.path,
        )
    }
}

pub trait Accessor {
    fn is_empty(&mut self) -> bool;
        // fn get_all_file_paths(&mut self) -> Keys<String, FileAspects>;
    fn tracks_files(&mut self) -> bool;
    fn tracks_file(&mut self, path: &OsStr) -> bool;
    fn track_file(&mut self, path: &OsStr, aspects: &FileAspects)
    -> Result<&mut Self, FileAlreadyTrackedError>;
    fn untrack_file(&mut self, path: &OsStr) -> Result<&mut Self, FileAlreadyTrackedError>;
    fn get_aspects(&mut self, path: &OsStr) -> Result<FileAspects, UntrackedFileError>;
}

impl Accessor for Index {
    fn is_empty(&mut self) -> bool {
        self.files.len() == 0
    }

    // fn get_all_file_paths(&mut self) -> Keys<String, FileAspects> {
    //     self.files.keys()
    // }

    fn tracks_files(&mut self) -> bool {
        self.files.len() > 0
    }

    fn tracks_file(&mut self, path: &OsStr) -> bool {
        if self.files.contains_key(path) {
            true
        }
        else {
            false
        }
    }
    
    fn track_file(&mut self, path: &OsStr, aspects: &FileAspects)
    -> Result<&mut Self, FileAlreadyTrackedError> {
        match self.files.insert(path.to_owned(), aspects.to_owned()) {
            None => Ok(self),
            Some(_) => Err(FileAlreadyTrackedError {
                path: path.to_owned(),
                index_struct: self.to_owned(),
                context_description: String::from("Adding new file to track."),
            })
        }
    }

    fn untrack_file(&mut self, path: &OsStr) -> Result<&mut Self, FileAlreadyTrackedError> {
        match self.files.remove_entry(path) {
            Some(_) => Ok(self),
            None => Err(FileAlreadyTrackedError {
                path: path.to_owned(),
                index_struct: self.to_owned(),
                context_description: String::from("Untracking a file.")
            })
        }
    }
    
    fn get_aspects(&mut self, path: &OsStr) -> Result<FileAspects, UntrackedFileError> {
        match self.files.get(path) {
            Some(aspects) => Ok(aspects.to_owned()),
            None => Err(UntrackedFileError {
                path: path.to_owned(),
                context_description: String::from("Getting file aspects by path.")
            })
        }
    }
}