use std::{
    io::{stdout, Write},
    process::Command,
};

pub fn exec_cmd(args: &[&str]) {
    let mut cmd = Command::new(args[0]);

    let exec_result = cmd.args(args[1..].iter()).output();

    match exec_result {
        Ok(output) => {
            let _ = stdout().write(&output.stdout);
        }
        Err(_) => {
            println!("{}: command not found", args[0])
        }
    }
}
