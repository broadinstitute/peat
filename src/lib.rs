use std::env;
use std::fs::File;
use std::io::{BufReader, Read};
use std::io;

use peatcode::evaluate::evaluate_declarations;
use peatcode::parse;
use peatcode::types::Bindings;
use util::error::Error;

use crate::peatcode::PeatCode;
use crate::util::error::Error::PeatError;

mod util;
mod peatcode;
mod substitute;
mod bash;

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
    println!("Peat file uses version {}", peat_code.version);
    print_declarations(&peat_code);
    println!("Template:");
    println!("{}", peat_code.body);
    let bindings_iter = evaluate_declarations(&peat_code);
    println!("Now evaluating");
    for bindings_result in bindings_iter {
        let bindings = bindings_result?;
        print_bindings(&bindings);
        let body_resolved = substitute::substitute(&peat_code.body, &bindings)?;
        match bash::run_bash_script(body_resolved) {
            Ok(_) => { println!("Process completed successfully.")}
            Err(error) => { println!("Process failed: {}", error)}
        }
    }
    println!("Done!");
    Ok(())
}

fn print_bindings(bindings: &Bindings) {
    let bindings_vec = bindings.to_vec();
    let mut entries_iter = bindings_vec.iter();
    print!("Bindings: ");
    if let Some((id, value)) = entries_iter.next() {
        print!("{} = {}", id, value);
        for (id, value) in entries_iter {
            print!(", {} = {}", id, value);
        }
        println!()
    } else {
        println!("[empty]");
    }
}

fn print_declarations(peat_code: &PeatCode) {
    let mut declaration_iter = peat_code.declarations.iter();
    print!("Declarations: ");
    if let Some(declaration) = declaration_iter.next() {
        print!("{}", declaration);
        for declaration in declaration_iter {
            print!(", {}", declaration);
        }
        println!();
    } else {
        println!("[none]");
    }
}