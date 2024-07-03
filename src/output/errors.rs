pub fn print_format_error(incorrect_format: String, available_formats: String) {
    println!(
        "The format {:?} is invalid, please use one of: \n{:?}",
        incorrect_format, available_formats
    );
}
