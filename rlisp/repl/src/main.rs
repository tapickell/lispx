use crate::evaluator::eval;
use crate::parser::parse_form;

use rustyline::error::ReadlineError;
use rustyline::Editor;

mod evaluator;
mod parser;
mod types;

fn main() {
    println!("LispR Version 0.0.0.0.1");
    println!("Interactive LispR - Press Ctrl+c to exit");

    let mut rl = Editor::<()>::new();
    if rl.load_history(".lispr_history").is_err() {
        println!("No previous history.")
    }

    loop {
        let readline = rl.readline("lispr> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                if line == "(:exit)" {
                    break;
                }
                match parse_form(line.as_str()) {
                    Ok((_input, parsed_line)) => {
                        let result = eval(parsed_line);
                        println!("{:?}", result);
                    }
                    Err(error) => {
                        println!("{} -> {}", error, line.as_str());
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("BREAK: (a)bort");
                break;
            }
            Err(ReadlineError::Eof) => break,
            Err(err) => {
                println!("Error: {}", err);
                break;
            }
        }
    }
    rl.save_history(".lispr_history").unwrap();
}
