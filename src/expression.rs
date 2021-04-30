use std::fmt::{Display, Formatter};
use std::fmt;

pub(crate) enum Expression {
    UIntLiteral(u64),
    BoolLiteral(bool),
    Variable(String)
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

pub(crate) struct Declaration {
    pub(crate) name: String,
    pub(crate) expression: Expression
}
