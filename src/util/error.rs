use std::fmt::{Display, Formatter};
use std::{fmt, result, io};
use crate::util::error::Error::{IoError, PeatError};
use clap::ErrorKind;

#[derive(Debug)]
pub enum Error {
    PeatError(String),
    IoError(io::Error),
    ClapError(clap::Error)
}

impl Error {
    pub fn is_real_error(&self) -> bool {
        match self {
            Error::PeatError(_) => true,
            Error::IoError(_) => true,
            Error::ClapError(clap_error) => {
                !matches!(clap_error.kind, ErrorKind::HelpDisplayed | ErrorKind::VersionDisplayed)
            }
        }
    }
}

impl Display for Error {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> result::Result<(), fmt::Error> {
        match self {
            Error::PeatError(message) => Display::fmt(message, formatter),
            Error::IoError(io_error) => Display::fmt(io_error, formatter),
            Error::ClapError(clap_error) => Display::fmt(clap_error, formatter)
        }
    }
}

impl From<io::Error> for Error {
    fn from(io_error: io::Error) -> Self {
        IoError(io_error)
    }
}

impl From<clap::Error> for Error {
    fn from(clap_error: clap::Error) -> Self {
        Error::ClapError(clap_error)
    }
}

impl From<&str> for Error {
    fn from(st: &str) -> Error { PeatError(String::from(st)) }
}

impl From<String> for Error {
    fn from(string: String) -> Error { PeatError(string) }
}