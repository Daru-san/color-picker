use std::env;
use std::fs;
use std::process::exit;

fn is_program_in_path(cmd: &str) -> bool {
    if let Ok(path) = env::var("PATH") {
        for p in path.split(':') {
            let p_str = format!("{}/{}", p, cmd);
            if fs::metadata(p_str).is_ok() {
                return true;
            }
        }
    }
    false
}

pub fn check_path(cmd: &str) {
    if is_program_in_path(cmd) {
        println!();
    } else {
        println!(
            "{:?} is not in your PATH, please ensure it is installed",
            cmd
        );
        exit(101);
    }
}
