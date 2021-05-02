use std::fmt::{Display, Formatter};
use std::fmt;
use crate::value::Value;
use crate::value::Value::UIntValue;
use crate::types::Bindings;
use crate::error::Error;

pub(crate) enum Expression {
    UIntLiteral(u64),
    Variable(String)
}

impl Expression {
    pub(crate) fn eval(&self, bindings: &Bindings) -> Result<Value, Error> {
        match self {
            Expression::UIntLiteral(ui) => Ok(UIntValue(*ui)),
            Expression::Variable(id) => {
                match bindings.get(id) {
                    Some(value) => Ok(*value),
                    None => Err(Error::from(format!("Unknown identifier {}.", id)))
                }
            }
        }
    }

}

impl Display for Expression {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Expression::UIntLiteral(integer) => Display::fmt(integer, formatter),
            Expression::Variable(name) => Display::fmt(name, formatter)
        }
    }
}
