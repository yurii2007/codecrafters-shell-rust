use std::{
    io::{stdout, Write},
    process::Command,
};

pub fn exec_cmd(cmd: &str, args: &Vec<String>) {
    let mut command = Command::new(cmd);

    let exec_result = command.args(args.iter()).output();

    match exec_result {
        Ok(output) => {
            let _ = stdout().write(&output.stdout);
        }
        Err(_) => {
            println!("{}: command not found", cmd)
        }
    }
}
