use std::env;

pub fn pwd() {
    match env::current_dir() {
        Ok(dir) => {
            println!("{}", dir.display())
        }
        Err(e) => {
            println!("error reading current dir:{}", e);
        }
    }
}
