#[allow(unused_imports)]
use std::io::{self, Write};
use std::process;

use echo_cmd::echo_cmd;
use exec_cmd::exec_cmd;
use type_cmd::type_cmd;

mod echo_cmd;
mod exec_cmd;
mod type_cmd;

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
            ["pwd", ..] => match std::env::current_dir() {
                Ok(dir) => {
                    println!("{}", dir.display())
                }
                Err(e) => {
                    println!("error reading current dir:{}", e);
                }
            },
            ["cd", args @ ..] => {
                let mut destination_path = String::from(args[0]);

                if destination_path.starts_with("~") {
                    if let Some(home_dir) = std::env::home_dir() {
                        destination_path =
                            destination_path.replace("~", &home_dir.to_string_lossy())
                    }
                }

                if let Err(_) = std::env::set_current_dir(&destination_path) {
                    println!("cd: {}: No such file or directory", args[0]);
                }
            }
            args => exec_cmd(args),
        }
    }
}
