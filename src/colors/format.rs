use crate::output::errors::print_format_error;
use std::process::exit;
use std::str::FromStr;

pub fn get_format(color_format: String) -> String {
    let available_formats: String = "hex, hsl, hsv, rgb or cmyk".to_string();

    let converted_format: &str = &String::from_str(&color_format).unwrap();

    let is_correct = matches!(converted_format, "hex" | "hsl" | "hsv" | "rgb" | "cmyk");

    if is_correct {
        color_format
    } else {
        print_format_error(color_format, available_formats);
        exit(404)
    }
}
