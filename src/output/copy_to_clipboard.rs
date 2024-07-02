use wl_clipboard_rs::copy::{MimeType, Options, Source};

pub fn run(color: String) {
    let clipboard = Options::new();

    let copy = clipboard.copy(
        Source::Bytes(color.to_string().into_bytes().into()),
        MimeType::Autodetect,
    );
    drop(copy);
}
