mod custom_button;

use gtk::prelude::*;
use gtk::{self, Application, ApplicationWindow, gio, glib};

const APP_ID: &str = "org.lovejoy.daily";

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    app.run()
}

fn build_ui(app: &Application) {
    let settings = gio::Settings::new(APP_ID);

    let switch = gtk::Switch::builder()
        .margin_top(48)
        .margin_bottom(48)
        .margin_start(48)
        .margin_end(48)
        .valign(gtk::Align::Center)
        .halign(gtk::Align::Center)
        .build();

    settings
        .bind("is-switch-enabled", &switch, "active")
        .build();

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Daily")
        .child(&switch)
        .build();

    window.present()
}
