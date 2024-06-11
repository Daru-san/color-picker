use clap::Parser;
use notify_rust::Notification;
use std::process::Command;
use std::process::Stdio;

use wl_clipboard_rs::copy::{MimeType, Options, Source};

/// A simple color picker wrapper for hyprpicker
#[derive(Parser, Debug)]
#[command(version,about,long_about = None)]
struct Args {
    /// Notify the user when picking the color
    #[arg(short, long)]
    notify: bool,

    /// Copy the color to the system clipboard
    #[arg(short, long)]
    clipboard: bool,
}

fn main() {
    let args = Args::parse();
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
    let notify = Notification::new()
        .summary("Color Picker")
        .body(&color)
        .show();

    println!("{}", color);
    if args.clipboard {
        drop(copy_to_clipboard);
        println!("Color {} successfully copied to your clipboard!", color);
    }
    if args.notify {
        drop(notify);
    }
}
