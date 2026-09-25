// Copyright 2026 Cade Weinberg.
// SPDX-License-Identifier: GPL-3.0-only

#ifndef DAILY_APP_APPLICATION_H
#define DAILY_APP_APPLICATION_H

#include <gtk/gtk.h>

G_BEGIN_DECLS

#define DAILY_TYPE_APPLICATION daily_application_get_type()
G_DECLARE_FINAL_TYPE(DailyApplication, daily_application, DAILY, APPLICATION, GtkApplication)

/**
 * @brief called to create a new application instance
 *
 * see also [GtkApplication](https://docs.gtk.org/gtk4/ctor.Application.new.html)
 */
DailyApplication *
daily_application_new(char const * identifier,
                      GApplicationFlags flags);

G_END_DECLS

#endif // !DAILY_APP_APPLICATION_H
