use std::fmt::{Display, Formatter};
use std::fmt;
use crate::{
    util::error::Error,
    util::error::Error::PeatError
};

pub(crate) enum Version {
    V1_0
}

mod names {
    pub const V1_0: &str = "1.0";
}

impl Display for Version {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Version::V1_0 => formatter.write_str(names::V1_0)
        }
    }
}

impl Version {
    pub(crate) fn parse(version_str: &str) -> Result<Version, Error> {
        match version_str {
            names::V1_0 => Ok(Version::V1_0),
            _ => Err(PeatError(format!("Unknown version {}", version_str)))
        }
    }
}