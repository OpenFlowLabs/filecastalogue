use std::{ffi::{OsStr}};
use crate::{error::{Error, ErrorKind, FcResult}};
use super::super::file_aspects::enums::TrackedFileAspects;
use super::{
    error::{FileAlreadyTrackedErrorPayload, UntrackedFileErrorPayload},
    model::Index};
    
pub trait Accessor {
    fn is_empty(&mut self) -> bool;
        // fn get_all_file_paths(&mut self) -> Keys<String, FileAspects>;
    fn tracks_files(&mut self) -> bool;
    fn tracks_file(&mut self, path: &OsStr) -> bool;
    fn track_file(&mut self, path: &OsStr, aspects: &TrackedFileAspects)
    -> FcResult<&mut Self>;
    fn untrack_file(&mut self, path: &OsStr) -> FcResult<&mut Self>;
    fn get_aspects(&mut self, path: &OsStr) -> FcResult<TrackedFileAspects>;
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
    
    fn track_file(&mut self, path: &OsStr, aspects: &TrackedFileAspects)
    -> FcResult<&mut Self> {
        match self.files.insert(path.to_owned(), aspects.to_owned()) {
            None => Ok(self),
            Some(_) => Err(error!(
                ErrorKind::FileAlreadyTracked,
                "Adding new file to track.",
                payload => FileAlreadyTrackedErrorPayload {
                    path: path.to_owned(),
                    index_struct: self.to_owned()
                }
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
                payload => FileAlreadyTrackedErrorPayload {
                    path: path.to_owned(),
                    index_struct: self.to_owned()
                }
            ))
        }
    }
    
    fn get_aspects(&mut self, path: &OsStr) -> FcResult<TrackedFileAspects> {
        match self.files.get(path) {
            Some(aspects) => Ok(aspects.to_owned()),
            None => Err(
                error!(
                    ErrorKind::UntrackedFile,
                    "Getting file aspects by path.",
                    payload => UntrackedFileErrorPayload {
                        path: path.to_owned()
                    }
                )
            )
        }
    }
}