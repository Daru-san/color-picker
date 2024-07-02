use clap::Parser;
use std::process::exit;
use std::process::Command;
use std::process::Stdio;
use std::str::FromStr;


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
    get_args();

    let color_format = get_format();
    let color = get_color(color_format);

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

fn get_format() -> String {
    let args = Args::parse();

    let format = args.format;

    let available_formats: String = "hex, hsl, hsv, rgb or cmyk".to_string();

    let converted_format: &str = &String::from_str(&format).unwrap();

    let is_correct = matches!(converted_format, "hex" | "hsl" | "hsv" | "rgb" | "cmyk");

    if is_correct {
        format
    } else {
        print_format_error(format, available_formats);
        exit(404)
    }
}

fn print_usage() {
    println!("Just run `color-picker` and it will copy the selected color to your clipboard");
    exit(0);
}
fn get_color(color_format: String) -> String {
    let proc = Command::new("hyprpicker")
        .arg("-f")
        .arg(color_format)
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
