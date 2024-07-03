use std::process::exit;

pub fn get_args(usage: bool) {
    if usage {
        print_usage();
    }
}

fn print_usage() {
    println!("Just run `color-picker` and it will copy the selected color to your clipboard");
    exit(0);
}
