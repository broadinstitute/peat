use crate::expression::Expression;
use std::fmt::{Display, Formatter};
use std::fmt;

pub(crate) struct Assignment {
    pub(crate) id: String,
    pub(crate) expression: Box<Expression>
}

impl Assignment {
    pub(crate) fn new(id: String, expression: Expression) -> Assignment {
        Assignment {
            id,
            expression: Box::new(expression)
        }
    }
}

pub(crate) enum Declaration {
    Assign(Assignment)
}

impl Display for Declaration {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Declaration::Assign(assignment) => {
                format!("{}={}", assignment.id, assignment.expression).fmt(f)
            }
        }
    }
}