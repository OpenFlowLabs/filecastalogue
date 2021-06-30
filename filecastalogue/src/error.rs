use std::{collections::HashMap, error::Error as StdError,fmt::{Debug, Display}, io};

pub trait Payload: Debug + Display {}

pub type FcResult<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum ErrorKind {
    RepoFileOperationFailed,
    FileAlreadyTracked,
    VersionEntryDoesNotExist,
    VersionEntryAlreadyExists,
    UntrackedFile,
    DoubleDotFileName,
    CollectionHandlerOperationFailed,
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
            ErrorKind::CollectionHandlerOperationFailed => "Collection handler operation failed.",
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

#[macro_export] 
macro_rules! payload {
    ($($key:expr, $value:expr),*) => {
        let tmp_payload = KeyValuePayload {}
        $(tmp_payload.store.insert($key, $value))*
        tmp_payload
    }
}

#[macro_export]
macro_rules! error {
    ($kind:expr, $context:expr, $payload:expr, $wrapped:expr) => {
        Error::new($kind, $context, $payload, $wrapped)
    };
    ($kind:expr, $context:expr) => {
        Error::new($kind, $context, None, None)
    };
    ($kind:expr, $context:expr, payload => $payload:expr) => {
        Error::new($kind, $context, Some(Box::new($payload)), None)
    };
    ($kind:expr, $context:expr, wrapped => $wrapped:expr) => {
        Error::new($kind, $context, None, $wrapped)
    };
}