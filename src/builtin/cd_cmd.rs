use std::env;

pub fn cd(args: &[String]) {
    let mut destination_path = args[0].clone();

    if destination_path.starts_with("~") {
        if let Some(home_dir) = env::home_dir() {
            destination_path = destination_path.replace("~", &home_dir.to_string_lossy())
        }
    }

    if let Err(_) = env::set_current_dir(&destination_path) {
        println!("cd: {}: No such file or directory", args[0]);
    }
}
