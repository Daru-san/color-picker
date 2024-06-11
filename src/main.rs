use clap::Parser;
use notify_rust::Notification;
use std::process::Command;

#[derive(Parser, Debug)]
#[command(version,about,long_about = None)]
struct Args {
    #[arg(short, long)]
    notify: bool,

    #[arg(short, long)]
    clipboard: bool,
}
fn main() {
    let args = Args::parse();
    let notify = Notification::new()
        .summary("Color Picker")
        .body(("Copied {} to clipboard", color))
        .show();

    Command::new("hyprpicker")
        .spawn()
        .expect("Command failed to execute");
    if args.notify {
        notify;
    }
}
