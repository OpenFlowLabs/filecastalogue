use std::{collections::HashMap, error::Error as StdError, fmt::{Debug, Display}, io, path::PathBuf};

pub trait Payload: Debug + Display {}

pub type FcResult<T> = std::result::Result<T, Error>;

#[derive(Debug)]
enum ConversionWasLossy {
    Yes,
    No
}

impl ConversionWasLossy {
    pub(crate) fn as_str(&self) -> &'static str {
        match *self {
            // We expect this to appear in front of the path,
            // hence the trailing space.
            ConversionWasLossy::Yes => concat!(
                "[WARNING: The path might be rendered incorrectly in this ",
                "error message, as lossless conversion to unicode wasn't ",
                "possible.] "
            ),
            // Since we want to be able to drop this enum directly into
            // error messages, the lossless case should behave as closely,
            // as possible to not existing while still being a str.
            // Hence the "".
            ConversionWasLossy::No => "",
        }
    }
}

impl Display for ConversionWasLossy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug)]
pub struct ErrorPathBuf {
    string: String,
    conversion_was_lossy: ConversionWasLossy
}
impl From<PathBuf> for ErrorPathBuf {
    fn from(path_buf: PathBuf) -> Self {
        let (string, conversion_was_lossy) =
            match path_buf.to_str() {
                Some(path) => (
                    String::from(path),
                    ConversionWasLossy::No
                ),
                None => (
                    String::from(path_buf.to_string_lossy()),
                    ConversionWasLossy::Yes
                )
        };
        Self {
            string: string,
            conversion_was_lossy: conversion_was_lossy
        }
    }
}

impl Display for ErrorPathBuf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}",
            self.conversion_was_lossy,
            self.string
        )
    }
}

#[derive(Debug)]
pub enum ErrorKind {
    RepoFileOperationFailed,
    FileAlreadyTracked,
    VersionEntryDoesNotExist,
    VersionEntryAlreadyExists,
    UntrackedFile,
    DoubleDotFileName,
    PathDoesNotExistInCollection,
    TestSetupSafetyCheckFailed,
    PuttingFileIntoCollectionFailed,
    Io,
    Serde
}

impl ErrorKind {
    pub(crate) fn as_str(&self) -> &'static str {
        match *self {
            ErrorKind::RepoFileOperationFailed => "Repo-file operation failed.",
            ErrorKind::FileAlreadyTracked => "File already tracked.",
            ErrorKind::VersionEntryDoesNotExist => "Version entry doesn't exist.",
            ErrorKind::VersionEntryAlreadyExists => "Version entry already exists.",
            ErrorKind::UntrackedFile => "Path for which there is no file tracked encountered.",
            ErrorKind::DoubleDotFileName => "Double-dot (..) file name encountered.",
            ErrorKind::PathDoesNotExistInCollection => "Path doesn't exist in collection.",
            ErrorKind::PuttingFileIntoCollectionFailed => "Putting file into collection failed.",
            ErrorKind::TestSetupSafetyCheckFailed => "Test setup safety check failed.",
            ErrorKind::Io => "Standard IO Error: std::io::Error.",
            ErrorKind::Serde => "Error with JSON (de)serialization: serde_json::Error.",
        }
    }
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.as_str()
        )
    }
}

#[derive(Debug)]
pub enum WrappedError {
    // We have to box our own error to prevent type recursion.
    Fc(Box<Error>),
    Io(io::Error),
    Serde(serde_json::Error)
}

// impl From<WrappedError> for &(dyn StdError + 'static) {
//     fn from(wrapped: WrappedError) -> Self {
//         match wrapped {
//             WrappedError::Fc(internal_error) => & *internal_error,
//             WrappedError::Io(io_error) => &io_error,
//             WrappedError::Serde(serde_error) => &serde_error
//         }
//     }
// }

// impl From<&WrappedError> for &(dyn StdError + 'static) {
//     fn from(wrapped: &WrappedError) -> Self {
//                 match wrapped {
//             WrappedError::Fc(internal_error) => &*internal_error,
//             WrappedError::Io(io_error) => &io_error,
//             WrappedError::Serde(serde_error) => &serde_error
//         }
//     }
// }

impl Display for WrappedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self
        )
    }
}

impl StdError for WrappedError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Self::Fc(internal_error) => internal_error.source(),
            Self::Io(io_error) => io_error.source(),
            Self::Serde(serde_error) => serde_error.source()
        }
    }
}

#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind,
    pub context: String,
    pub payload: Option<Box<dyn Payload>>,
    pub wrapped: Option<WrappedError>,
}

impl Error {
    pub fn new<ContextRef: AsRef<str>>(
        kind: ErrorKind,
        context: ContextRef,
        payload: Option<Box<dyn Payload>>,
        wrapped: Option<WrappedError>
    ) -> Self {
        Self {
            kind: kind,
            context: context.as_ref().to_owned(),
            payload: payload,
            wrapped: wrapped
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Error: {}.",
            self.kind,
        )
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match &self.wrapped {
            // What we really return here are the result(s) of
            // WrappedError's (and maybe ExternalError's) From implementation.
            Some(e) => e.source(),
            None => None
        }
    }

    fn cause(&self) -> Option<&dyn StdError> {
        self.source()
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::new(
            ErrorKind::Io, 
            "<converted from std::io::Error>",
            None,
            Some(WrappedError::Io(e))
        )
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::new(
            ErrorKind::Serde,
            "<converted from serde_json::Error>",
            None,
            Some(WrappedError::Serde(e))
        )
    }
}

pub trait KeyValuePayloadValue: Debug + Display {}

pub struct KeyValuePayload {
    store: HashMap<String, Box<dyn Debug>>
}

impl KeyValuePayload {
    pub fn new() -> Self {
        Self {
            store: HashMap::new()
        }
    }
    pub fn add
    <
        KeyRef: AsRef<str>,
    >
    (mut self, key: KeyRef, value: Box<(dyn Debug)>) -> Self {
        self.store.insert(key.as_ref().to_owned(), value);
        self
    }
}

impl Debug for KeyValuePayload {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?}",
            self.store
        )
    }
}

impl Display for KeyValuePayload {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?}",
            self.store
        )
    }
}

impl Payload for KeyValuePayload {}
impl Payload for &mut KeyValuePayload {}
impl Payload for String {}
impl Payload for &str {}

#[macro_export] 
macro_rules! payload {
    ($($key:expr, $value:expr),*) => {
        KeyValuePayload::new()
        $(.add($key, $value))*
    }
}

#[macro_export]
macro_rules! error {
    (
        $kind:expr,
        $context:expr,
        $payload:expr,
        $wrapped:expr) => {
            Error::new($kind, $context, Some(Box::new($payload)), Some($wrapped))
    };
    (
        $kind:expr,
        $context:expr) => {
            Error::new($kind, $context, None, None)
    };
    (
        $kind:expr, $context:expr, payload => $payload:expr) => {
            Error::new($kind, $context, Some(Box::new($payload)), None)
    };
    (
        $kind:expr, $context:expr, wrapped => $wrapped:expr) => {
            Error::new($kind, $context, None, Some($wrapped))
    };
}