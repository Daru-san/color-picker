use clap::Parser;

mod checks;
mod colors;
mod output;
mod subcommands;

use colors::{format, picker};
use output::{copy_to_clipboard, notification};
use subcommands::usage;

/// A simple color picker wrapper for hyprpicker
#[derive(Parser, Debug)]
#[command(author,version,about,long_about = None)]
struct Args {
    /// Get a tutorial on how to use
    #[arg(short, long)]
    usage: bool,

    /// The output format of the color
    #[arg(short, long, default_value = "hex")]
    format: String,
}

fn main() {
    let args = Args::parse();
    usage::get_args(args.usage);

    let color_format = format::get_format(args.format);

    let message = format!("{:?} has been copied to your clipboard", color);

    println!("{}", message);
    copy_to_clipboard::run(color);
    notification::run(message);
}

fn get_args() {
    let args = Args::parse();
    if args.usage {
        print_usage();
    }
}


fn print_usage() {
    println!("Just run `color-picker` and it will copy the selected color to your clipboard");
    let mut color = String::from_utf8(command.stdout).unwrap();
    color.truncate(color.len() - 1);
    check_color(color.clone());
    color
}
