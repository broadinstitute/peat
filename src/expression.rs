use std::fmt::{Display, Formatter};
use std::fmt;
use crate::value::Value;
use crate::value::Value::UIntValue;
use crate::types::Bindings;
use crate::error::Error;

pub(crate) trait Expression: Display {
    fn eval(&self, bindings: &Bindings) -> Result<Value, Error>;
}

pub(crate) struct UIntLiteral {
    value: u64,
}

pub(crate) struct Variable {
    id: String,
}

impl UIntLiteral {
    pub(crate) fn new(value: u64) -> UIntLiteral { UIntLiteral { value } }
}

impl Variable {
    pub(crate) fn new(id: String) -> Variable { Variable { id } }
}

impl Expression for UIntLiteral {
    fn eval(&self, _: &Bindings) -> Result<Value, Error> { Ok(UIntValue(self.value)) }
}

impl Expression for Variable {
    fn eval(&self, bindings: &Bindings) -> Result<Value, Error> {
        match bindings.get(&self.id) {
            Some(value) => Ok(value),
            None => Err(Error::from(format!("Unknown identifier {}.", self.id)))
        }
    }
}

impl Display for UIntLiteral {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.value, formatter)
    }
}

impl Display for Variable {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.id, formatter)
    }
}
