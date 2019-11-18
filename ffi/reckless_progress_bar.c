
            #include <gtk/gtk.h>
            #include "reckless_progress_bar.h"
            
            G_DEFINE_TYPE( RecklessProgressBar, reckless_progress_bar, GTK_TYPE_PROGRESS_BAR )
            
            static void reckless_progress_bar_class_init( RecklessProgressBarClass* klass ) {
            	GtkWidgetClass *widget_class = GTK_WIDGET_CLASS(klass);
            	widget_class->get_preferred_width = reckless_progress_bar_get_preferred_width;
            	widget_class->get_preferred_height = reckless_progress_bar_get_preferred_height;
            }
            static void reckless_progress_bar_init( RecklessProgressBar* self ) {}
            
            static void reckless_progress_bar_get_preferred_width(GtkWidget *widget, int *minimal, int *natural) {
            	*minimal = *natural = 1;
            }
            static void reckless_progress_bar_get_preferred_height(GtkWidget *widget, int *minimal, int *natural) {
            	*minimal = *natural = 1;
            }
            GtkWidget* reckless_progress_bar_new (void) {
              return g_object_new (RECKLESS_PROGRESS_BAR_TYPE, NULL);
            }
        