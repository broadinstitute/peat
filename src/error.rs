use std::fmt::{Display, Formatter};
use std::{fmt, result, io};
use crate::error::Error::{IoError, PeatError};

pub enum Error {
    PeatError(String),
    IoError(io::Error),
}

impl Display for Error {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> result::Result<(), fmt::Error> {
        match self {
            Error::PeatError(message) => Display::fmt(message, formatter),
            Error::IoError(io_error) => Display::fmt(io_error, formatter)
        }
    }
}

impl From<io::Error> for Error {
    fn from(io_error: io::Error) -> Self {
        IoError(io_error)
    }
}

impl From<&str> for Error {
    fn from(st: &str) -> Error { PeatError(String::from(st)) }
}