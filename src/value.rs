use std::fmt::{Display, Formatter};
use std::fmt;

#[derive(Copy, Clone)]
pub(crate) enum Value {
    UIntValue(u64)
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Value::UIntValue(ui) => { Display::fmt(ui, f) }
        }
    }
}