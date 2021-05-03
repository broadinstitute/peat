use crate::peatcode::PeatCode;
use crate::types::{Bindings, get_empty_bindings};
use crate::error::Error;
use crate::declaration::Declaration;

pub(crate) fn evaluate_declarations(peat_code: &PeatCode) -> Result<Bindings, Error> {
    let mut bindings = get_empty_bindings();
    for declaration in &peat_code.declarations {
        bindings = evaluate(declaration, bindings)?;
    }
    Ok(bindings)
}

fn evaluate(declaration: &Declaration, bindings: Bindings)
    -> Result<Bindings, Error> {
    match declaration {
        Declaration::Assign(assignment) => {
            let id = assignment.id.clone();
            let value = assignment.expression.eval(&bindings)?;
            Ok(bindings.with(id, value))
        }
    }
}