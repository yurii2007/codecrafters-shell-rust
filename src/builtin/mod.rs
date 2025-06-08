use cd_cmd::cd;
use echo_cmd::echo;
use exit_cmd::exit;
use pwd_cmd::pwd;
use type_cmd::type_cmd;

mod cd_cmd;
mod echo_cmd;
mod exit_cmd;
mod pwd_cmd;
mod type_cmd;

pub fn exec_builtin(cmd: &str, args: &Vec<String>) -> bool {
    match cmd {
        "echo" => echo(args),
        "type" => type_cmd(args),
        "exit" => exit(args),
        "pwd" => pwd(),
        "cd" => cd(args),
        _ => return false,
    };

    true
}
