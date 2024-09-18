pub fn format_error(incorrect_format: String, available_formats: String) {
    println!(
        "The format {:?} is invalid, please use one of: \n{:?}",
        incorrect_format, available_formats
    );
}
pub fn path_error(command: &str) {}
pub fn unknown_error() {}
