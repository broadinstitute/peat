use crate::peatcode::PeatCode;
use crate::types::{Bindings, get_empty_bindings};
use crate::error::Error;
use crate::declaration::Declaration;
use std::iter;
use std::iter::Once;

pub(crate) fn evaluate_declarations(peat_code: &PeatCode) -> Result<Once<Bindings>, Error> {
    let mut bindings = get_empty_bindings();
    for declaration in &peat_code.declarations {
        bindings = evaluate_old(declaration, bindings)?;
    }
    Ok(iter::once(bindings))
}

fn evaluate_old(declaration: &Declaration, bindings: Bindings)
                -> Result<Bindings, Error> {
    match declaration {
        Declaration::Assign(assignment) => {
            let id = assignment.id.clone();
            let value = assignment.expression.eval(&bindings)?;
            Ok(bindings.with_value(id, value))
        }
    }
}

fn evaluate(declaration: &'static Declaration, bindings_iter: Box<dyn Iterator<Item=Bindings>>)
            -> Box<dyn Iterator<Item=Bindings>> {
    let iter = match declaration {
        Declaration::Assign(assignment) => {
            let id = assignment.id.clone();
            bindings_iter.map(move |bindings|{
                let value =
                    assignment.expression.eval(&bindings)
                        .expect(format!("Cannot evaluate {}", id).as_str());
                bindings.with_value(id.clone(), value)
            })
        }
    };
    Box::new(iter)
}