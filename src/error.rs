use std::fmt::{Display, Formatter};
use std::{fmt, result};

pub enum Error {
    PeatError(String)
}

impl Display for Error {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> result::Result<(), fmt::Error> {
        match self {
            Error::PeatError(message) => message.fmt(formatter),
        }
    }
}