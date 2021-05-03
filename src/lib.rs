mod error;
mod peatcode;
mod expression;
mod value;
mod parse;
mod version;
mod types;
mod tokenize;
mod declaration;
mod evaluate;
mod substitute;
mod matryoshka;

use error::Error;
use std::env;
use crate::error::Error::PeatError;
use std::io::{BufReader, Read};
use std::io;
use std::fs::File;
use crate::peatcode::PeatCode;
use crate::evaluate::evaluate_declarations;

fn get_input_file_name() -> Result<Option<String>, Error> {
    let mut args = env::args();
    let input_file = Ok(args.nth(1));
    if let Some(superfluous_arg) = args.next() {
        return Err(PeatError(format!("Unexpected argument {}", superfluous_arg)));
    }
    input_file
}

fn get_peat_code() -> Result<PeatCode, Error> {
    let input_file_name = get_input_file_name()?;
    let source: Box<dyn Read> = match input_file_name {
        Some(file_path) => Box::new(File::open(file_path)?),
        None => Box::new(io::stdin())
    };
    let input_buf_reader = BufReader::new(source);
    parse::parse_input(input_buf_reader)
}

pub fn run() -> Result<(), Error> {
    let peat_code = get_peat_code()?;
    let bindings = evaluate_declarations(&peat_code)?;
    println!("Parsed some PeatCode!");
    println!("Peat version is {}", peat_code.version);
    for declaration in &peat_code.declarations {
        println!("{}", declaration);
    }
    println!("After evaluation:");
    for (id, value)  in bindings.to_vec().iter() {
        println!("{}={}", id, value);
    }
    println!("Body original:");
    println!("{}", peat_code.body);
    let body_resolved = substitute::substitute(&peat_code.body, &bindings);
    println!("Body resolved:");
    println!("{}", body_resolved?);
    Ok(())
}