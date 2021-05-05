use std::fmt::{Display, Formatter};
use std::fmt;
use crate::value::Value;
use crate::value::Value::UIntValue;
use crate::types::Bindings;
use crate::error::Error;

pub(crate) enum Type {
    UInt
}

pub(crate) trait Expression: Display {
    fn eval(&self, bindings: &Bindings) -> Result<Value, Error>;
    fn get_type(&self) -> Type;
    fn as_typed(&self) -> AsTyped;
}

pub(crate) enum AsTyped<'a> {
    AsUInt(&'a dyn UIntExpression)
}

impl AsTyped<'_> {
    fn get_type(&self) -> Type {
        match self {
            AsTyped::AsUInt(_) => { Type::UInt }
        }
    }
}

pub(crate) trait UIntExpression: Expression {
    fn eval_int(&self, bindings: &Bindings) -> Result<u64, Error>;
}

pub(crate) struct UIntLiteral {
    value: u64,
}

pub(crate) struct UIntVariable {
    id: String,
}

impl UIntLiteral {
    pub(crate) fn new(value: u64) -> UIntLiteral { UIntLiteral { value } }
}

impl UIntVariable {
    pub(crate) fn new(id: String) -> UIntVariable { UIntVariable { id } }
}

impl Expression for UIntLiteral {
    fn eval(&self, _: &Bindings) -> Result<Value, Error> { Ok(UIntValue(self.value)) }
    fn get_type(&self) -> Type { Type::UInt }
    fn as_typed<'a>(&'a self) -> AsTyped<'a> { AsTyped::AsUInt::<'a>(self) }
}

impl UIntExpression for UIntLiteral {
    fn eval_int(&self, _bindings: &Bindings) -> Result<u64, Error> { Ok(self.value) }
}

impl Expression for UIntVariable {
    fn eval(&self, bindings: &Bindings) -> Result<Value, Error> {
        match bindings.get(&self.id) {
            Some(value) => Ok(value),
            None => Err(Error::from(format!("Unknown identifier {}.", self.id)))
        }
    }

    fn get_type(&self) -> Type { Type::UInt }
    fn as_typed<'a>(&'a self) -> AsTyped<'a> { AsTyped::AsUInt::<'a>(self) }
}

impl UIntExpression for UIntVariable {
    fn eval_int(&self, bindings: &Bindings) -> Result<u64, Error> {
        match bindings.get(&self.id) {
            Some(UIntValue(ui)) => Ok(ui),
            // Some(value) =>
            //     Err(Error::from(format!("Expected unsigned int, but got {}.", value))),
            None => Err(Error::from(format!("Unknown identifier {}.", self.id)))
        }
    }
}

impl Display for UIntLiteral {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.value, formatter)
    }
}

impl Display for UIntVariable {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.id, formatter)
    }
}
