#[allow(unused_imports)]
use std::io::{self, Write};

use builtin::exec_builtin;
use exec_cmd::exec_cmd;

mod builtin;
mod exec_cmd;

fn main() {
    let stdin = io::stdin();

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        let input = input.trim();

        let (cmd, args): (String, Vec<String>) = match input.split_once(" ") {
            Some((cmd, input)) => {
                let mut args: Vec<String> = Vec::new();
                let mut current_arg = String::new();
                let mut is_quoted = false;

                for char in input.chars() {
                    match (char, is_quoted) {
                        ('\'', _) => is_quoted = !is_quoted,
                        (' ', _) => {
                            if !current_arg.is_empty() {
                                args.push(std::mem::take(&mut current_arg));
                            }
                        }
                        (c, _) => current_arg.push(c),
                    }
                }

                if !current_arg.is_empty() {
                    args.push(current_arg);
                }

                (cmd.to_string(), args)
            }
            None => {
                if input.is_empty() {
                    continue;
                } else {
                    (input.to_string(), Vec::new())
                }
            }
        };

        let is_builtin = exec_builtin(&cmd, &args);

        if !is_builtin {
            exec_cmd(&cmd, &args);
        }
    }
}
