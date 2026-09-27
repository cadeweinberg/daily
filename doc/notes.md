<!--
 Copyright 2026 Cade Weinberg.
 SPDX-License-Identifier: GPL-3.0-only
-->

# Notes


## GTK4

when do we inherit from GObject?
- We want to use a certain widget, with added state and overriden virtual functions
- We want to pass a Rust object to a function, and the function expects a GObject
- We want to add properties or signals to a GObject

