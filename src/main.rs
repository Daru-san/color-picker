use clap::Parser;
use notify_rust::Notification;
use std::process::Command;
use std::process::Stdio;

use wl_clipboard_rs::copy::{MimeType, Options, Source};

/// A simple color picker wrapper for hyprpicker
#[derive(Parser, Debug)]
#[command(author,version,about,long_about = None)]
struct Args {}

fn main() {
    let proc = Command::new("hyprpicker")
        .stdout(Stdio::piped())
        .output()
        .unwrap();

    let color = String::from_utf8(proc.stdout).unwrap();

    let opts = Options::new();

    let copy_to_clipboard = opts.copy(
        Source::Bytes(color.to_string().into_bytes().into()),
        MimeType::Autodetect,
    );

    let message = color + " has been copied to your clipboard";

    let notify = Notification::new()
        .summary("Color Picker")
        .body(&message)
        .show();

    println!("{}", message);
    drop(copy_to_clipboard);
    drop(notify);
}
