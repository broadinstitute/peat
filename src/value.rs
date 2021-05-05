use std::fmt::{Display, Formatter};
use std::fmt;
use crate::error::Error;

#[derive(Copy, Clone)]
pub(crate) enum Value {
    UIntValue(u64),
    UIntRangeValue { from: u64, until: u64 },
}

impl Value {
    pub(crate) fn as_int(&self) -> Result<u64, Error> {
        match self {
            Value::UIntValue(ui) => Ok(*ui),
            Value::UIntRangeValue{ from, until} =>
                Err(Error::from(
                    format!("Expected integer, but got range {} .. {}", from, until)
                ))
        }
    }

    pub(crate) fn new_int(ui: u64) -> Value { Value::UIntValue(ui) }

    pub(crate) fn new_range(from: u64, until: u64) -> Value {
        Value::UIntRangeValue { from, until }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Value::UIntValue(ui) => { Display::fmt(ui, f) }
            Value::UIntRangeValue{ from, until } => {
                Display::fmt(format!("{} .. {}", from, until).as_str(), f)
            }
        }
    }
}