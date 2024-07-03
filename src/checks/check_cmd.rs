use std::env;
use std::fs;

fn is_program_in_path(cmd: &str) -> bool {
    if let Ok(path) = env::var("PATH") {
        for p in path.split(":") {
            let p_str = format!("{}/{}", p, cmd);
            if fs::metadata(p_str).is_ok() {
                return true;
            }
        }
    }
    false
}

pub fn check() {
    let cmd = "hyprpicker";
    if is_program_in_path(cmd) {
        println!("");
    } else {
        println!(
            "{:?} is not in your PATH, please ensure it is installed",
            cmd
        );
    }
}
