use std::{env, os::unix::fs::PermissionsExt, path::Path};

pub fn type_cmd(args: &[&str]) {
    if args.is_empty() {
        return;
    }

    match args[0] {
        "type" | "echo" | "exit" | "pwd" => println!("{} is a shell builtin", args[0]),
        _ => check_executable(args[0]),
    }
}

fn check_executable(arg: &str) {
    let path = env::var("PATH");

    match path {
        Ok(path_str) => {
            let path_dirs: Vec<&str> = path_str.split(":").collect();

            for path_dir_str in path_dirs.iter() {
                let file_path_str = format!("{}/{}", path_dir_str, arg);
                let file_path = Path::new(&file_path_str);

                if !file_path.exists() || file_path.is_dir() {
                    continue;
                } else if file_path.is_file() {
                    if let Ok(file_metadata) = file_path.metadata() {
                        if file_metadata.permissions().mode() & 0o111 == 0 {
                            continue;
                        } else {
                            println!("{} is {}", arg, file_path.to_str().unwrap_or(""));
                            return;
                        }
                    }
                }
            }

            println!("{}: not found", arg);
        }
        Err(_) => println!("{}: not found", arg),
    }
}
