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

        let (command, user_input) = extract_command(&mut input);

        match command {
            "echo" => println!("{}", user_input.unwrap_or("")),
            "exit" => process::exit(0),
            _ => println!("{}: command not found", input.trim()),
        }
    }
}

fn extract_command(input: &mut String) -> (&str, Option<&str>) {
    let parsed_input = input.trim().split_once(" ");

    match parsed_input {
        Some((command, user_input)) => (command, Some(user_input)),
        None => (input.trim(), None),
    }
}
