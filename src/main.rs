#[cfg(target_os = "macos")]
use cocoa::appkit::NSApplication;

use gio::prelude::*;
use gtk::prelude::*;

use gtk::{Align, Application, ApplicationWindow, Button, ButtonBox, Entry, Label};

#[cfg(target_os = "macos")]
fn macos_init() {
    unsafe {
        // https://stackoverflow.com/questions/47497878/gtk-window-present-does-not-move-window-to-foreground
        cocoa::appkit::NSApp().activateIgnoringOtherApps_(true);
    }
}

fn main() {
    let application = Application::new(Some("com.sokoide.gtk-rust-demo"), Default::default())
        .expect("failed to initialize GTK application");

    application.connect_activate(|app| {
        let window = ApplicationWindow::new(app);
        window.set_title("Hoge");
        window.set_default_size(320, 200);

        let vbox = gtk::Box::new(gtk::Orientation::Vertical, 10);
        window.add(&vbox);

        let label = Label::new(Some("Hello, I'm a sample GTK3 app in Rust!"));
        label.set_halign(Align::Start);
        vbox.add(&label);

        let bbox = ButtonBox::new(gtk::Orientation::Horizontal);
        vbox.add(&bbox);

        let button = Button::with_label("Click me!");
        button.connect_clicked(|_| {
            println!("Clicked!");
        });
        bbox.add(&button);

        let entry = Entry::new();
        vbox.add(&entry);

        #[cfg(target_os = "macos")]
        macos_init();

        window.show_all();
    });

    println!("1");
    application.run(&[]);
}
