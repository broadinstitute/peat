use code::types::Bindings;
use util::error::Error;
use code::{PeatCode, evaluate};
use script_files::ScriptNameGenerator;
use crate::config::Config;

pub mod util;
mod code;
mod substitute;
mod bash;
mod script_files;
mod config;

pub fn lib_main() {
    match config::get_config() {
        Ok(peat_config) => {
            match run(peat_config) {
                Err(error) => {
                    eprintln!("Error: {}", error)
                }
                Ok(()) => println!("Done!")
            }
        }
        Err(error) => {
            if error.is_real_error() {
                eprintln!("Error: {}", error)
            } else {
                println!("{}", error)
            }
        }
    }
}

fn run(peat_config: Config) -> Result<(), Error> {
    let peat_code = code::get_peat_code(&peat_config.input_file)?;
    println!("Peat file uses version {}", peat_code.version);
    print_declarations(&peat_code);
    if !peat_config.parse_only {
        let bindings_iter = evaluate::evaluate_declarations(&peat_code);
        println!("Now evaluating");
        let mut script_name_gen = ScriptNameGenerator::from_temp_dir()?;
        for bindings_result in bindings_iter {
            let bindings = bindings_result?;
            print_bindings(&bindings);
            if !peat_config.dry_run {
                let script_path = script_name_gen.next();
                let body_resolved = substitute::substitute(&peat_code.body, &bindings)?;
                match bash::run_bash_script(script_path.as_path(), &body_resolved) {
                    Ok(_) => { println!("Process completed successfully.") }
                    Err(error) => { eprintln!("Process failed: {}", error) }
                }
            }
        }
    }
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