use std::io::{self, Write};

use builtin::exec_builtin;
use exec_cmd::exec_cmd;
use parse_args::parse_args;

mod builtin;
mod exec_cmd;
mod parse_args;

fn main() {
    let stdin = io::stdin();

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        let mut args = parse_args(input);

        if let Some(cmd) = args.pop_front() {
            let args: Vec<String> = Vec::from(args);

            let is_builtin = exec_builtin(&cmd, &args);

            if !is_builtin {
                exec_cmd(&cmd, &args);
            }
        } else {
            continue;
        }
    }
}
