use crate::peatcode::PeatCode;
use crate::types::{Bindings, BindingsIterator};
use crate::error::Error;
use crate::declaration::{Declaration, Assignment};
use crate::types;

pub(crate) fn evaluate_declarations(peat_code: &PeatCode) -> BindingsIterator {
    let mut bindings_iter = types::new_bindings_iter();
    for declaration in &peat_code.declarations {
        bindings_iter = evaluate(declaration, bindings_iter);
    }
    bindings_iter
}

fn bindings_for_assign(bindings_result: Result<Bindings, Error>,
                       assignment: &Assignment) -> Result<Bindings, Error> {
    let bindings = bindings_result?;
    let id = assignment.id.clone();
    let value = assignment.expression.eval(&bindings)?;
    Ok(bindings.with_value(id.clone(), value))
}

fn evaluate<'a>(declaration: &'a Declaration, bindings_iter: BindingsIterator<'a>)
                -> BindingsIterator<'a> {
    let iter = match declaration {
        Declaration::Assign(assignment) => {
            bindings_iter.map(move |bindings_result| {
                bindings_for_assign(bindings_result, assignment)
            })
        }
    };
    Box::new(iter)
}