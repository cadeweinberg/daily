// Copyright 2026 Cade Weinberg.
// SPDX-License-Identifier: GPL-3.0-only

#include <glib.h>
#include <gtk/gtk.h>

#define DAILY_TYPE_APPLICATION daily_application_get_type()
G_DECLARE_FINAL_TYPE(DailyApplication, daily_application, DAILY, APPLICATION, GtkApplication)

/**
 * @brief called on the primary application instance,
 * immediately after registration.
 */
void 
daily_application_startup (DailyApplication *self,
                           gpointer          data);

/**
 * @brief called on the primary application instance, 
 * immediately after the main loop terminates.
 */
void
daily_application_shutdown (DailyApplication *self,
                            gpointer          data);

/**
 * @brief called to show the window of the application.
 *
 * after initialization and startup, when the application 
 * is started directly.
 */
void 
daily_application_activate (DailyApplication *self,
                            gpointer          data);

/**
 * @brief called to show the window of the application.
 *
 * after initialization and startup, when the application 
 * is started via a file association.
 */
void 
daily_application_open (DailyApplication *self,
                        gpointer          files,
                        gint              n_files,
                        gchar            *hint,
                        gpointer          data);


                        
