#[allow(unused_imports)]
use std::io::{self, Write};
use std::process;

fn main() {
    let stdin = io::stdin();

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        match input.trim() {
            "exit 0" => process::exit(0),
            _ => println!("{}: command not found", input.trim()),
        }
    }
}
