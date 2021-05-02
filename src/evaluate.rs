use crate::peatcode::PeatCode;
use crate::types::{Bindings, get_empty_bindings};
use crate::error::Error;
use crate::declaration::Declaration;

pub(crate) fn evaluate_declarations(peat_code: &PeatCode) -> Result<Bindings, Error> {
    let mut bindings = get_empty_bindings();
    for declaration in &peat_code.declarations {
        match declaration {
            Declaration::Assign(assignment) => {
                let id = assignment.id.clone();
                let value = assignment.expression.eval(&bindings)?;
                let previous_value =
                    bindings.insert(id, value);
                if let Some(_) = previous_value {
                    return
                        Err(Error::from(
                            format!("{} has already been previously declared", value)
                        ))
                }
            }
        }
    }
    Ok(bindings)
}