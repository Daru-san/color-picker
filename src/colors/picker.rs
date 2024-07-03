use crate::checks::{check_cmd, check_color};
use std::process::Command;
use std::process::Stdio;

pub fn get_color(color_format: String) -> String {
    let cmd = "hyprpicker";
    check_cmd::check_path(cmd);
    let command = Command::new(cmd)
        .arg("-f")
        .arg(color_format)
        .stdout(Stdio::piped())
        .output()
        .unwrap();

    let mut color = String::from_utf8(command.stdout).unwrap();
    color.truncate(color.len() - 1);
    check_color::check_color(color.clone());
    color
}
