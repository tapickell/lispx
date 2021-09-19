use rustyline::error::ReadlineError;
use rustyline::Editor;

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
                if line == "(:exit)" {
                    break;
                }
                rl.add_history_entry(line.as_str());
                println!("#=> {}", line);
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
