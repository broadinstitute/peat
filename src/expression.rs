use std::fmt::{Display, Formatter};
use std::fmt;
use crate::value::Value;
use crate::value::Value::{UIntValue, BoolValue};
use crate::types::Bindings;

pub(crate) enum Expression {
    UIntLiteral(u64),
    BoolLiteral(bool),
    Variable(String)
}

enum EvalResult {
    Unchanged,
    Optimized(Expression),
    Resolved(Value)
}

impl Expression {
    fn eval(&self, bindings: Bindings) -> EvalResult {
        match self {
            Expression::UIntLiteral(ui) => EvalResult::Resolved(UIntValue(*ui)),
            Expression::BoolLiteral(boo) => EvalResult::Resolved(BoolValue(*boo)),
            Expression::Variable(name) => {
                match bindings.get(name) {
                    Some(value) => EvalResult::Resolved(*value),
                    None => EvalResult::Unchanged
                }
            }
        }
    }

}

impl Display for Expression {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Expression::UIntLiteral(integer) => Display::fmt(integer, formatter),
            Expression::BoolLiteral(boolean) => Display::fmt(boolean,formatter),
            Expression::Variable(name) => Display::fmt(name, formatter)
        }
    }
}
