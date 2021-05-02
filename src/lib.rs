mod error;
mod peatcode;
mod expression;
mod value;
mod parse;
mod version;
mod types;

use error::Error;
use std::env;
use crate::error::Error::PeatError;
use std::io::{BufReader, Read};
use std::io;
use std::fs::File;

fn get_input_file_name() -> Result<Option<String>, Error> {
    let mut args = env::args();
    let input_file = Ok(args.nth(1));
    if let Some(superfluous_arg) = args.next() {
        return Err(PeatError(format!("Unexpected argument {}", superfluous_arg)));
    }
    input_file
}

pub fn run() -> Result<(), Error> {
    let input_file_name = get_input_file_name()?;
    let source: Box<dyn Read> = match input_file_name {
      Some(file_path) => Box::new(File::open(file_path)?),
        None => Box::new(io::stdin())
    };
    let input_buf_reader = BufReader::new(source);
    let peat_code = parse::parse_input(input_buf_reader)?;
    println!("Parsed some PeatCode!");
    println!("Peat version is {}", peat_code.version);
    for declaration in peat_code.declarations {
        println!("{}={}", declaration.name, declaration.expression);
    }
    println!("Body:");
    println!("{}", peat_code.body);
    Ok(())
}