use clap::Args;
use std::process::exit;
use std::str::FromStr;
use crate::output::errors::print_format_error;

fn get_format(Args: struct) -> String {
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
