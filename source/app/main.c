/**
 * Copyright 2026 cade.
 * SPDX-License-Identifier: GPL-3.0-only
 */

#include <gtk/gtk.h>

#include "common/config.h"

static void 
activate (GtkApplication *application,
          gpointer        data) {
    GtkWidget *window = NULL;

    window = GTK_WIDGET(gtk_application_get_active_window(application));
    if (window == NULL) {
        window = gtk_application_window_new(application);
    }
    
    gtk_window_set_title(GTK_WINDOW(window), DAILY_WINDOW_DEFAULT_TITLE);
    gtk_window_set_default_size(GTK_WINDOW(window), 
                                DAILY_WINDOW_DEFAULT_WIDTH,
                                DAILY_WINDOW_DEFAULT_HEIGHT);
    gtk_window_present(GTK_WINDOW(window));
}


int main(int argc, char **argv) {
    // Create an application context
    // Create an application window
    // fill the window with widgets which allow for:
    //      questions listing
    //      questions selection
    //      questions scheduling
    //      reviewing answered questions
    //          graphs
    //          history
    GtkApplication *application = NULL;
    int status = 0;

    application = gtk_application_new("org.gtk.example", G_APPLICATION_DEFAULT_FLAGS);
    g_signal_connect (application, "activate", G_CALLBACK(activate), NULL);
    
    status = g_application_run(G_APPLICATION(application), argc, argv);
    g_object_unref (application);

    return status;
}