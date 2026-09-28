// Copyright 2026 Cade Weinberg.
// SPDX-License-Identifier: GPL-3.0-only

use gio::Settings;
use gtk::subclass::prelude::*;
use gtk::{ApplicationWindow, gio, glib};
use std::cell::OnceCell;

#[derive(Default)]
pub struct Window {
    pub settings: OnceCell<Settings>,
}

#[glib::object_subclass]
impl ObjectSubclass for Window {
    const NAME: &'static str = "DailyApplicationWindow";
    type Type = super::Window;
    type ParentType = ApplicationWindow;
}

impl ObjectImpl for Window {
    fn constructed(&self) {
        self.parent_constructed();
        let obj = self.obj();
        obj.setup_settings();
        obj.load_window_size();
    }
}

impl WidgetImpl for Window {}

impl WindowImpl for Window {
    fn close_request(&self) -> glib::Propagation {
        self.obj()
            .save_window_size()
            .expect("Failed to save window state");

        glib::Propagation::Proceed
    }
}

impl ApplicationWindowImpl for Window {}


