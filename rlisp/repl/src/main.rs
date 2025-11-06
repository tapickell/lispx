use rlisp_core::process_str;
use rustyline::error::ReadlineError;
use rustyline::Editor;

fn main() {
    println!("LispR Version 0.0.0.0.2");
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
                match process_str(&line.as_str()) {
                    Ok(result) => {
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
