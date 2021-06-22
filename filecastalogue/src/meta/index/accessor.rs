use std::{ffi::{OsStr, OsString}, fmt};
use crate::{error::{Error, ErrorKind, FcResult, Payload}, meta::index::model::{Index}};

use super::model::FileAspects;

pub struct FileAlreadyTrackedErrorPayload {
    pub path: OsString,
    pub index_struct: Index,
}

impl fmt::Debug for FileAlreadyTrackedErrorPayload {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Error: The file at path \"{:?}\" {} {:?}.",
            self.path,
            "is already tracked, with the following entry:",
            self.index_struct,
        )
    }
}

impl fmt::Display for FileAlreadyTrackedErrorPayload {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Error: The file at path \"{:?}\" is already tracked.",
            self.path,
        )
    }
}

impl Payload for FileAlreadyTrackedErrorPayload {}


pub struct UntrackedFileErrorPayload {
    pub path: OsString,
}

impl fmt::Debug for UntrackedFileErrorPayload {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "There is no file tracked for path \"{:?}\".",
            self.path
        )
    }
}

impl fmt::Display for UntrackedFileErrorPayload {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "There is no file tracked for path \"{:?}\".",
            self.path
        )
    }
}

impl Payload for UntrackedFileErrorPayload {}

pub trait Accessor {
    fn is_empty(&mut self) -> bool;
        // fn get_all_file_paths(&mut self) -> Keys<String, FileAspects>;
    fn tracks_files(&mut self) -> bool;
    fn tracks_file(&mut self, path: &OsStr) -> bool;
    fn track_file(&mut self, path: &OsStr, aspects: &FileAspects)
    -> FcResult<&mut Self>;
    fn untrack_file(&mut self, path: &OsStr) -> FcResult<&mut Self>;
    fn get_aspects(&mut self, path: &OsStr) -> FcResult<FileAspects>;
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
    -> FcResult<&mut Self> {
        match self.files.insert(path.to_owned(), aspects.to_owned()) {
            None => Ok(self),
            Some(_) => Err(error!(
                ErrorKind::FileAlreadyTracked,
                "Adding new file to track.",
                payload => Some(Box::new(FileAlreadyTrackedErrorPayload {
                    path: path.to_owned(),
                    index_struct: self.to_owned()
                }))
            ))
            // Err(FileAlreadyTrackedError {
            //     path: path.to_owned(),
            //     index_struct: self.to_owned(),
            //     context_description: String::from("Adding new file to track."),
            // })
        }
    }

    fn untrack_file(&mut self, path: &OsStr) -> FcResult<&mut Self> {
        match self.files.remove_entry(path) {
            Some(_) => Ok(self),
            None => Err(error!(
                ErrorKind::FileAlreadyTracked,
                "Untracking a file.",
                payload => Some(Box::new(FileAlreadyTrackedErrorPayload {
                    path: path.to_owned(),
                    index_struct: self.to_owned()
                }))
            ))
        }
    }
    
    fn get_aspects(&mut self, path: &OsStr) -> FcResult<FileAspects> {
        match self.files.get(path) {
            Some(aspects) => Ok(aspects.to_owned()),
            None => Err(
                error!(
                    ErrorKind::UntrackedFile,
                    "Getting file aspects by path.",
                    payload => Some(Box::new(UntrackedFileErrorPayload {
                        path: path.to_owned()
                    }))
                )
            )
        }
    }
}