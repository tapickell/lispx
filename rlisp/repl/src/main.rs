use ctrlc;
use std::io::{self, Write};
use std::process;

fn main() {
    ctrlc::set_handler(move || {
        println!("BREAK: (a)bort");
        process::exit(1);
    })
    .expect("Error setting ctrl-c handler");

    println!("LispR Version 0.0.0.0.1");
    println!("Press Ctrl+c to Exit");
    print!("lispr> ");

    io::stdout().flush().unwrap();

    let mut buffer = String::new();

    loop {
        io::stdin().read_line(&mut buffer).unwrap();
        println!("#=> {}", buffer.trim());
        print!("lispr> ");
        io::stdout().flush().unwrap();
    }
}
