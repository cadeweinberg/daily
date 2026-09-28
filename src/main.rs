mod custom_button;

use std::thread;
use std::time::Duration;

use gtk::prelude::*;
use gtk::{self, Application, ApplicationWindow, Button, gio, glib};

const APP_ID: &str = "org.lovejoy.daily";

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    app.run()
}

fn build_ui(app: &Application) {
    let button = Button::builder()
        .label("Press me!")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let (sender, receiver) = async_channel::bounded(1);
    button.connect_clicked(move |_| {
        glib::spawn_future_local(glib::clone!(
            #[strong]
            sender,
            async move {
                sender
                    .send_blocking(false)
                    .expect("The channel needs to be open.");
                glib::timeout_future_seconds(5).await;
                sender
                    .send_blocking(true)
                    .expect("The channel needs to be open.");
            }
        ));
    });

    glib::spawn_future_local(glib::clone!(
        #[weak]
        button,
        async move {
            while let Ok(enable_button) = receiver.recv().await {
                button.set_sensitive(enable_button);
            }
        }
    ));

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Daily")
        .child(&button)
        .build();

    window.present()
}
