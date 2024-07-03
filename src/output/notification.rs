use notify_rust::Notification;

pub fn run(message: String) {
    let notify = Notification::new()
        .summary("Color Picker")
        .body(&message)
        .appname("Color Picker")
        .show();
    drop(notify);
}
