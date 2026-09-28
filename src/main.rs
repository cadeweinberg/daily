
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
    let button_1 = CustomButton::new();
    let button_2 = CustomButton::new();

    button_1
        .bind_property("number", &button_2, "number")
        // How to transform "number" from `button_1` to "number" or `button_2`
        .transform_to(|_, number: i32| {
            let incremented_number = number + 1;
            return Some(incremented_number.to_value());
        })
        // How to transform "number" from `button_2` to "number" of `button_1`
        .transform_from(|_, number: i32| {
            let decremented_number = number - 1;
            return Some(decremented_number.to_value());
        })
        .bidirectional()
        .sync_create()
        .build();
    
    button_1.connect_number_notify(|button|{
        println!("The current number of `button_1` is {}.", button.number());
    });

    let gtk_box = gtk::Box::builder()
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .valign(gtk::Align::Center)
        .halign(gtk::Align::Center)
        .spacing(12)
        .orientation(gtk::Orientation::Vertical)
        .build();
    gtk_box.append(&button_1);
    gtk_box.append(&button_2);

    let window = gtk::ApplicationWindow::builder()
        .application(app)
        .title("Daily")
        .child(&gtk_box)
        .build();

    window.present();
}


