// Copyright 2026 Cade Weinberg.
// SPDX-License-Identifier: GPL-3.0-only

use std::cell::Cell;

use glib::Properties;
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

// Object holding the state
#[derive(Properties, Default)]
#[properties(wrapper_type = super::CustomButton)]
pub struct CustomButton {
    #[property(get, set)]
    number: Cell<i32>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for CustomButton {
    const NAME: &'static str = "MyGtkAppCustomButton";
    type Type = super::CustomButton;
    type ParentType = gtk::Button;
}

// Trait shared by all GObjects
#[glib::derived_properties]
impl ObjectImpl for CustomButton {
    fn constructed(&self) {
        self.parent_constructed();

        // Bind label to number
        // `SYNC_CREATE` ensures that the label will be immediately set
        let obj = self.obj();
        obj.bind_property("number", obj.as_ref(), "label")
            .sync_create()
            .build();
    }
}

// Trait shared by all widgets
impl WidgetImpl for CustomButton {}

// Trait shared by all buttons
impl ButtonImpl for CustomButton {
    fn clicked(&self) {
        let incremented_number = self.obj().number() + 1;
        self.obj().set_number(incremented_number);
    }
}