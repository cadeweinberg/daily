
use gtk::glib;
use gtk::subclass::prelude::*;

// Object holding the state
#[derive(Default)]
pub struct CustomButton;

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for CustomButton {
    const NAME: &'static str = "DailyCustomButton";
    type Type = super::CustomButton;
    type ParentType = gtk::Button;
}

impl ObjectImpl for CustomButton {}

impl WidgetImpl for CustomButton {}

impl ButtonImpl for CustomButton {}


