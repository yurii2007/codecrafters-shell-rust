#[allow(unused_imports)]
use std::io::{self, Write};
use std::process;

fn main() {
    let stdin = io::stdin();

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        let input = input.trim();

        let command: Vec<&str> = input.split(" ").collect();

        match command.as_slice() {
            [""] => continue,
            ["echo", args @ ..] => echo_cmd(args),
            ["type", args @ ..] => type_cmd(args),
            ["exit", code] => process::exit(code.parse().unwrap_or(0)),
            _ => println!("{input}: command not found"),
        }
    }
}

fn echo_cmd(args: &[&str]) {
    println!("{}", args.join(" "));
}

fn type_cmd(args: &[&str]) {
    if args.is_empty() {
        return;
    }

    match args[0] {
        "type" | "echo" | "exit" => println!("{} is a shell builtin", args[0]),
        _ => println!("{}: not found", args[0]),
    }
}
