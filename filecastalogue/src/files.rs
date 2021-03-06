use std::{fmt::{self, Debug, Display}, io::{Read, Write}};
use crate::error::{FcResult, Payload};

pub mod blob;
pub mod tracked_ordinary_blob;
pub mod index;
pub mod state;
pub mod hashable;
pub mod tracked_ordinary_blob_collection;
pub mod index_collection;
pub mod state_collection;

pub struct AccessRepoFileErrorPayload {
    pub offending_action: OffendingAction,
    pub repo_file_variety: String,
    pub stream_info: Option<String>,
    pub buf_info: Option<String>,
    pub payload: Option<Box<dyn Payload>>
}

impl AccessRepoFileErrorPayload {
    pub fn new<VRef: AsRef<str>>(
        offending_action: OffendingAction,
        repo_file_variety: VRef,
        stream_info: Option<String>,
        buf_info: Option<String>,
        payload: Option<Box<dyn Payload>>
    ) -> Self {
        Self {
            offending_action: offending_action,
            repo_file_variety: repo_file_variety.as_ref().to_owned(),
            stream_info: stream_info,
            buf_info: buf_info,
            payload: payload
        }
    }
}

#[derive(Debug)]
pub enum OffendingAction {
    LoadingRepoFile,
    SavingRepoFile,
}

impl OffendingAction {
    pub(crate) fn as_str(&self) -> &'static str {
        match self {
            Self::LoadingRepoFile => "loading repo-file",
            Self::SavingRepoFile => "saving repo-file"
        }
    }
}

impl Display for OffendingAction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Debug for AccessRepoFileErrorPayload {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Failed {}, file variety: {}.{}{}",
            self.offending_action,
            self.repo_file_variety,
            // The string we received is expected to already contain
            // the {:?} of the objects we're receiving info about as
            // deemed necessary at the call site.
            match &self.stream_info {
                Some(stream_info) => format!("Stream info: {}", stream_info),
                None => String::from("") // matching format!'s String typing.
            },
            match &self.buf_info {
                Some(buf_info) => format!("Buf(Reader/Writer) info: {}", buf_info),
                None => String::from("") // matching format!'s String typing.
            }
        )
    }
}

impl Display for AccessRepoFileErrorPayload {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Failed {}, file variety: {}.",
            self.offending_action,
            self.repo_file_variety,
        )
    }
}

impl Payload for AccessRepoFileErrorPayload {}

// TODO: Find a way to run this macro inside of this here module.
//  Or break it out into a different one. ^^"
#[macro_export]
macro_rules! access_repo_file_error_payload {
    (
        $offending_action:expr,
        variety => $variety:expr,
        stream_info => $stream_info:expr,
        buf_info => $buf_info:expr,
    ) => {
        Some(Box::new(AccessRepoFileErrorPayload::new(
            $offending_action,
            $variety,
            $stream_info,
            $buf_info,
        )))
    };
}

// TODO: Find a way to not have all that macro copy pasta. ^^"
//  Problem: It can't find the access_repo_file_error_payload! macro
//  defined just above, which would somewhat reduce the code duplication
//  here.

#[macro_export]
macro_rules! access_repo_file_error {
    (
        $offending_action:expr,
        context => $context:expr,
        variety => $variety:expr,
        wrapped => $wrapped:expr
    ) => {
        Error::new(
            ErrorKind::RepoFileOperationFailed,
            $context,
            Some(Box::new(AccessRepoFileErrorPayload::new(
                $offending_action,
                $variety,
                None,
                None,
                None
            ))),
            Some($wrapped)
        )
    };
    (
        $offending_action:expr,
        context => $context:expr,
        variety => $variety:expr,
        wrapped => $wrapped:expr,
        payload => $payload:expr
    ) => {
        Error::new(
            ErrorKind::RepoFileOperationFailed,
            $context,
            Some(Box::new(AccessRepoFileErrorPayload::new(
                $offending_action,
                $variety,
                None,
                None,
                Some($payload)
            ))),
            Some($wrapped)
        )
    };
    (
        $offending_action:expr,
        context => $context:expr,
        variety => $variety:expr,
        wrapped => $wrapped:expr,
        payload => $payload:expr,
        buf => $buf_info:expr
    ) => {
        Error::new(
            ErrorKind::RepoFileOperationFailed,
            $context,
            Some(Box::new(AccessRepoFileErrorPayload::new(
                $offending_action,
                $variety,
                None,
                Some(format!("{:?}", $buf_info)),
                Some($payload)
            ))),
            Some($wrapped)
        )
    };
    (
        $offending_action:expr,
        context => $context:expr,
        variety => $variety:expr,
        wrapped => $wrapped:expr,
        payload => $payload:expr,
        buf => $buf_info:expr,
        stream => $stream_info:expr
    ) => {
        Error::new(
            ErrorKind::RepoFileOperationFailed,
            $context,
            Some(Box::new(AccessRepoFileErrorPayload::new(
                $offending_action,
                $variety,
                Some(format!("{:?}", $stream_info)),
                Some(format!("{:?}", $buf_info)),
                Some($payload)
            ))),
            Some(WrappedError::Serde($wrapped))
        )
    };
    }

/**
Models the abstract notion of a "file" in the repo, regardless
whether the backend structure of the store that implements it is
actually based on files. The mental model is that things like
indexes, the state "file" and blobs are stored in files of
their own, but how that actually happens, and whether it's even
files at the end of the day doesn't really concern us here.
 */
pub trait RepoFile {
    fn load(self: &mut Self, readable: &mut (dyn Read)) -> FcResult<()>;
    fn save(self: &mut Self, writeable: &mut dyn Write) -> FcResult<()>;
}

pub trait RepoFileCollection {
    
}

