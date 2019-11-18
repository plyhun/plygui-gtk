
            #include <gtk/gtk.h>
            #include "reckless_scrolled_window.h"
            
            G_DEFINE_TYPE( RecklessScrolledWindow, reckless_scrolled_window, GTK_TYPE_SCROLLED_WINDOW )
            
            static void reckless_scrolled_window_class_init( RecklessScrolledWindowClass* klass ) {
            	GtkWidgetClass *widget_class = GTK_WIDGET_CLASS(klass);
            	widget_class->get_preferred_width = reckless_scrolled_window_get_preferred_width;
            	widget_class->get_preferred_height = reckless_scrolled_window_get_preferred_height;
            }
            static void reckless_scrolled_window_init( RecklessScrolledWindow* self ) {}
            
            static void reckless_scrolled_window_get_preferred_width(GtkWidget *widget, int *minimal, int *natural) {
            	*minimal = *natural = 1;
            }
            static void reckless_scrolled_window_get_preferred_height(GtkWidget *widget, int *minimal, int *natural) {
            	*minimal = *natural = 1;
            }
            GtkWidget* reckless_scrolled_window_new (void) {
              return g_object_new (RECKLESS_SCROLLED_WINDOW_TYPE, NULL);
            }
        