use clap::Parser;
use notify_rust::Notification;
use std::process::exit;
use std::process::Command;
use std::process::Stdio;

use wl_clipboard_rs::copy::{MimeType, Options, Source};

/// A simple color picker wrapper for hyprpicker
#[derive(Parser, Debug)]
#[command(author,version,about,long_about = None)]
struct Args {
    #[arg(short, long)]
    usage: bool,
}

fn main() {
    get_args();
    let color = get_color();
    let message = format!("{:?} has been copied to your clipboard", color);

    println!("{}", message);
    copy_to_clipboard(color);
    notify(message);
}

fn get_args() {
    let args = Args::parse();
    if args.usage {
        print_usage();
    }
}

fn print_usage() {
    println!("Just run `color-picker` and it will copy the selected color to your clipboard");
    exit(0);
}
fn get_color() -> String {
    let proc = Command::new("hyprpicker")
        .stdout(Stdio::piped())
        .output()
        .unwrap();

    let mut color = String::from_utf8(proc.stdout).unwrap();
    color.truncate(color.len() - 1);
    check_color(color.clone());
    color
}

fn check_color(color: String) {
    if color.is_empty() {
        exit(0);
    }
}

fn copy_to_clipboard(color: String) {
    let clipboard = Options::new();

    let run = clipboard.copy(
        Source::Bytes(color.to_string().into_bytes().into()),
        MimeType::Autodetect,
    );
    drop(run);
}

fn notify(message: String) {
    let run = Notification::new()
        .summary("Color Picker")
        .body(&message)
        .appname("Color Picker")
        .auto_icon()
        .show();
    drop(run);
}
