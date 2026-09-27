
use std;
use gtk::prelude::*;
use gtk::glib;

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
    let up = gtk::Button::builder()
        .label("Up!")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let down = gtk::Button::builder()
        .label("Down!")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let display = gtk::Button::builder()
        .label("0")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let number = std::rc::Rc::new(std::cell::Cell::new(0));

    up.connect_clicked(
        glib::clone!(#[weak] number,
            move |_| {
                number.set(number.get() + 1);
            }
        )
    );

    down.connect_clicked(
        glib::clone!(#[weak] number,
            move |_| {
                number.set(number.get() - 1);
            }
        )
    );

    display.connect_clicked(
        glib::clone!(#[strong] number,
                     #[weak] display,
                     move |_| {
                display.set_label(&number.get().to_string());
            }
        )
    );

    let container = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .build();

    container.append(&up);
    container.append(&down);
    container.append(&display);

    let window = gtk::ApplicationWindow::builder()
        .application(app)
        .title("Daily")
        .child(&container)
        .build();

    window.present();
}


