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

When can you spawn an `async` task on the glib main event loop?
(i.e. in a context which is called from Daily `main`)
glib should be able to with:
- functions which come from the glib ecosystem
- functions which don't depend on any runtimes (tokio)
    which only depend on futures from the stdlib (futures-io, futures-core, etc)
- depend on the async-std or smol runtimes
- have cargo features that let them depend on async-std or smol instead of tokio

When should you use async tasks, and when should you use full threads?
- if the task spends it's time calculating, it is CPU-bound, that usually 
    implies using a full thread and communication via channels
- if the task is IO-bound, Web requests, file-io, etc
    that usually implies an async task. though benchmarking is recommended.