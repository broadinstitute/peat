mod error;
mod peatcode;
mod expression;
mod value;
mod parse;
mod version;

use error::Error;
use std::env;
use crate::error::Error::PeatError;

fn get_input_file_name() -> Result<String, Error> {
    let mut args = env::args();
    let input_file = match args.nth(1) {
        Some(input_file) => Ok(input_file),
        None => Err(PeatError("Missing input file argument.".to_string()))
    };
    if let Some(superfluous_arg) = args.next() {
        return Err(PeatError(format!("Unexpected argument {}", superfluous_arg)));
    }
    input_file
}

pub fn run() -> Result<(), Error> {
    let input_file_name = get_input_file_name()?;
    let peat_code = parse::parse_file(input_file_name.as_str())?;
    println!("Parsed some PeatCode!");
    println!("Peat version is {}", peat_code.version);
    for declaration in peat_code.declarations {
        println!("{}={}", declaration.name, declaration.expression);
    }
    println!("Body:");
    println!("{}", peat_code.body);
    Ok(())
}