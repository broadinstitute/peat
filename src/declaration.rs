use crate::expression::{Expression, UIntRangeExpression};
use std::fmt::{Display, Formatter};
use std::fmt;

pub(crate) struct Assignment {
    pub(crate) id: String,
    pub(crate) expression: Box<dyn Expression>
}

pub(crate) struct Iteration {
    pub(crate) id: String,
    pub(crate) expression: Box<dyn UIntRangeExpression>
}

impl Assignment {
    pub(crate) fn new(id: String, expression: Box<dyn Expression>) -> Assignment {
        Assignment { id, expression }
    }
}

impl Iteration {
    pub(crate) fn new(id: String, expression: Box<dyn UIntRangeExpression>) -> Iteration {
        Iteration { id, expression }
    }
}

pub(crate) enum Declaration {
    Assign(Assignment),
    Iterate(Iteration)
}

impl Display for Declaration {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Declaration::Assign(assignment) => {
                format!("{} = {}", assignment.id, assignment.expression).fmt(f)
            }
            Declaration::Iterate(iteration) => {
                format!("{} <- {}", iteration.id, iteration.expression).fmt(f)
            }
        }
    }
}