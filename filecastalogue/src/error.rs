use std::fmt::{Debug, Display};

pub trait Payload: Debug + Display {}

pub type FcResult<T> = std::result::Result<T, Error>;

pub enum ErrorKind {
    RepoFileOperationFailed,
    InvalidFileName
}

impl ErrorKind {
    pub(crate) fn as_str(&self) -> &'static str {
        match *self {
            ErrorKind::RepoFileOperationFailed => "Repo-file operation failed.",
            ErrorKind::InvalidFileName => "Invalid file name."

        }
    }
}

pub struct Error {
    pub kind: ErrorKind,
    pub context: String,
    pub payload: Option<Box<dyn Payload>>,
    pub wrapped: Option<Box<dyn std::error::Error>>,
}

impl Error {
    pub fn new<ContextRef: AsRef<str>>(
        kind: ErrorKind, 
        context: ContextRef,
        payload: Option<Box<dyn Payload>>,
        wrapped: Option<Box<dyn std::error::Error>>
    ) -> Self {
        Self {
            kind: kind,
            context: context.as_ref().to_owned(),
            payload: payload,
            wrapped: wrapped
        }
    }
}
