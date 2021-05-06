use crate::peatcode::PeatCode;
use crate::types::{Bindings, BindingsIterator};
use crate::error::Error;
use crate::declaration::{Declaration, Assignment, Iteration};
use crate::types;
use crate::value::Value;

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

fn bindings_iter_for_iteration(bindings_result: Result<Bindings, Error>,
                               iteration: &Iteration) -> BindingsIterator {
    let id = iteration.id.clone();
    match bindings_result {
        Ok(bindings) => {
            match iteration.expression.eval_range(&bindings) {
                Ok(range) => {
                    let iter = range.to_range().map(move |i|{
                        Ok(bindings.clone().with_value(id.clone(), Value::new_int(i)))
                    });
                    Box::new(iter)
                }
                Err(error) => {
                    Box::new(std::iter::once(Err(error)))
                }
            }
        }
        Err(error) => {
            Box::new(std::iter::once(Err(error)))
        }
    }
}

fn evaluate<'a>(declaration: &'a Declaration, bindings_iter: BindingsIterator<'a>)
                -> BindingsIterator<'a> {
    match declaration {
        Declaration::Assign(assignment) => {
            let iter = bindings_iter.map(move |bindings_result| {
                bindings_for_assign(bindings_result, assignment)
            });
            Box::new(iter)
        }
        Declaration::Iterate(iteration) => {
            let iter = bindings_iter.flat_map(move |bindings_result| {
                bindings_iter_for_iteration(bindings_result, iteration)
            });
            Box::new(iter)
        }
    }
}