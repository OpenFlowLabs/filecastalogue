
use std::collections::HashMap;
use std::error::Error as StdError;
use std::fmt::{Debug, Display};
use std::io;
use std::path::PathBuf;
use std::backtrace::Backtrace;
use std::process::Termination;
use std::ops::FromResidual;
use std::convert::Infallible;


pub trait Payload: Debug + Display {}

pub type FcResult<T> = std::result::Result<T, Error>;

pub struct FcTestResult<T> {
    result: FcResult<T>
}

/// Return type for tests which enables the printing of errors with linebreaks.
/// Use `.into` for the `Result` returned by the test to make this work,
/// unless it's returned by `?`.
/// A `From` implementation needs to exist for errors returned by `?`. For
/// `FcResult` such an implementation is already provided.
impl<T> FcTestResult<T> {
    fn new(result: FcResult<T>) -> Self {
        Self {
            result
        }
    }

    fn print(&self) -> () {
        match &self.result {
            Ok(_) => (),
            Err(error) => println!("{:#?}", error)
        };
    }

    fn new_and_print(result: FcResult<T>) -> Self {
        let this = Self {
            result
        };
        this.print();
        this
    }
}

impl<T> From<FcResult<T>> for FcTestResult<T> {
    fn from(result: FcResult<T>) -> Self {
        FcTestResult::new_and_print(result)
    }
}

impl<T: Termination> Termination for FcTestResult<T> {
    fn report(self) -> std::process::ExitCode {
        self.print();
        self.result.report()
    }
}

impl<T, E> FromResidual<Result<Infallible, E>> for FcTestResult<T>
where Error: From<E> {
    fn from_residual(residual: Result<Infallible, E>) -> Self {
        FcTestResult::new(FcResult::from_residual(residual))
    }
}

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
                "possible.] " // <- trailing space.
            ),
            // Since we want to be able to drop this enum directly into
            // error messages, the lossless case should behave as closely
            // as possible to not existing while still being a str,
            // hence the "".
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
    pub backtrace: Backtrace,
}

impl Error {
    pub fn new<ContextRef: AsRef<str>>(
        kind: ErrorKind,
        context: ContextRef,
        payload: Option<Box<dyn Payload>>,
        wrapped: Option<WrappedError>
    ) -> Self {
        // TODO: Look into pretty-printing errors the `?` (has to
        // work in tests).
        let error = Self {
            kind: kind,
            context: context.as_ref().to_owned(),
            payload: payload,
            wrapped: wrapped,
            backtrace: Backtrace::capture()
        };
        // println!("{:#?}", &error);
        return error
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