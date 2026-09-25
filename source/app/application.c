/**
 * Copyright 2026 Cade Weinberg.
 * SPDX-License-Identifier: GPL-3.0-only
 */

#include "app/application.h"
#include <gio/gio.h>

/**
 * @note name derived from G_DECLARE_FINAL_TYPE in the header file.
 * 
 */
struct _DailyApplication {
    GObject base;
    // #TODO: SQLite view
};

G_DEFINE_TYPE(DailyApplication, daily_application, G_TYPE_OBJECT)


/**
 * @brief called on the primary application instance,
 * immediately after registration.
 */
static void 
daily_application_startup (DailyApplication *self,
                           gpointer          data) {
    // #NOTE: The standard handler for GApplication::startup
    // calls gtk_init(). So we must chain up to the default 
    // handler before anything else
    GApplicationClass *base_class = G_APPLICATION_CLASS(self);
    base_class->startup(G_APPLICATION(self));

    // #NOTE: since object construction is not allowed to fail,
    // we cannot initialize the SQL database view here. We must 
    // provide an object method which opens the DB for read/write
    // and provides a point of failure.
}

/**
 * @brief called on the primary application instance, 
 * immediately after the main loop terminates.
 */
static void
daily_application_shutdown (DailyApplication *self,
                            gpointer          data) {}

/**
 * @brief called to show the window of the application.
 *
 * after initialization and startup, when the application 
 * is started directly.
 */
static void 
daily_application_activate (DailyApplication *self,
                            gpointer          data) {}

/**
 * @brief called to show the window of the application.
 *
 * after initialization and startup, when the application 
 * is started via a file association.
 */
static void 
daily_application_open (DailyApplication *self,
                        gpointer          files,
                        gint              n_files,
                        gchar            *hint,
                        gpointer          data) {}


/**
 * @note dispose is meant to free all types referenced from this object
 * which might themselves hold a reference to self. Generally, the most 
 * simple solution is to unref all members on which you own a reference.
 *
 * @note It is possible that object methods might be invoked after 
 * dispose is run and before finalize runs. GObject does not consider 
 * this to be a program error: you must gracefully detect this and 
 * neither crash nor warn the user, by having a disposed instance revert
 * to an inert state.
 */
static void 
daily_application_dispose(GObject *gobject) {
    // we don't hold any members which are pointers to other GObjects
    // or GObject derived classes.

    // we always chain up to the parent class.
    G_OBJECT_CLASS (daily_application_parent_class)->dispose(gobject);
}

/**
 * @note finalize is meant to free all members which are dynamically 
 * allocated by this class.
 */
static void 
daily_application_finalize(GObject *gobject) {
    // we always chain up to the parent class.
    G_OBJECT_CLASS (daily_application_parent_class)->finalize(gobject);
}

static void 
daily_application_class_init(DailyApplicationClass *self) {
    GObjectClass *object_class = G_OBJECT_CLASS (self);
    // initialize virtual methods on class
    object_class->dispose  = daily_application_dispose;
    object_class->finalize = daily_application_finalize;

    // #QUESTION so, we also need to hook up the handlers for startup,
    // shutdown, activate, and open here? or is that handled in main?
    // or is it handled in the init function?
}

static void 
daily_application_init(DailyApplication *self) {
    // #GUESS upon object initialization, the init method is called
    // at some point. This method *seems* like a decent place to 
    // initialize the signal methods for Application initialization.
    // This will remove some boilerplate from main, which in turn makes 
    // DailyApplication easier to construct properly, within main.
    g_signal_connect(self, "startup", G_CALLBACK(daily_application_startup), NULL);
    g_signal_connect(self, "shutdown", G_CALLBACK(daily_application_shutdown), NULL);
    g_signal_connect(self, "activate", G_CALLBACK(daily_application_activate), NULL);
    g_signal_connect(self, "open", G_CALLBACK(daily_application_open), NULL);
}

DailyApplication *
daily_application_new(char const * identifier,
                      GApplicationFlags flags) {


                      }