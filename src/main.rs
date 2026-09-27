
mod custom_button;

use std;
use gtk::prelude::*;
use gtk::glib;

use custom_button::CustomButton;

const APP_ID: &str = "org.lovejoy.daily";

fn main() -> glib::ExitCode {
    // mains job in a gtk application is to run a GtkApplication.
    // first we create our application
    // then we run it
    let app = gtk::Application::builder()
        .application_id(APP_ID)
        .build();

    app.connect_activate(build_ui);

    return app.run();
}

fn build_ui(app: &gtk::Application) {
    let button = CustomButton::new();
    button.set_margin_top(12);
    button.set_margin_bottom(12);
    button.set_margin_start(12);
    button.set_margin_end(12);

    let window = gtk::ApplicationWindow::builder()
        .application(app)
        .title("Daily")
        .child(&button)
        .build();

    window.present();
}


