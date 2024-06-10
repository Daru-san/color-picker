use clap::Parser;
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

    Command::new("hyprpicker")
        .spawn()
        .expect("Command failed to execute");
}
