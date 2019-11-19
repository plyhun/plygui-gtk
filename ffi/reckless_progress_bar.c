
            #include <gtk/gtk.h>
            #include "reckless_progress_bar.h"
            
            G_DEFINE_TYPE( RecklessProgressBar, reckless_progress_bar, GTK_TYPE_PROGRESS_BAR )
            
            static void reckless_progress_bar_class_init( RecklessProgressBarClass* klass ) {
            	GtkWidgetClass *widget_class = GTK_WIDGET_CLASS(klass);
            	widget_class->get_preferred_width = reckless_progress_bar_get_preferred_width;
            	widget_class->get_preferred_height = reckless_progress_bar_get_preferred_height;
            	widget_class->get_preferred_height_for_width = reckless_progress_bar_get_preferred_height_for_width;
                widget_class->get_preferred_width_for_height = reckless_progress_bar_get_preferred_width_for_height;
                widget_class->get_preferred_height_and_baseline_for_width = reckless_progress_bar_get_preferred_height_and_baseline_for_width;
                // widget_class->get_preferred_size = reckless_progress_bar_get_preferred_size;
            }
            static void reckless_progress_bar_init( RecklessProgressBar* self ) {}
            
            static void reckless_progress_bar_get_preferred_width(GtkWidget *widget, int *minimal, int *natural) {
            	*minimal = *natural = 1;
            }
            static void reckless_progress_bar_get_preferred_height(GtkWidget *widget, int *minimal, int *natural) {
            	*minimal = *natural = 1;
            }
            
            static void reckless_progress_bar_get_preferred_height_for_width (GtkWidget *widget, int value, int *minimal, int *natural) {
                *minimal = *natural = 1;
            }
            static void reckless_progress_bar_get_preferred_width_for_height (GtkWidget *widget, int value, int *minimal, int *natural) {
                *minimal = *natural = 1;
            }
            static void reckless_progress_bar_get_preferred_height_and_baseline_for_width (GtkWidget *widget, int width, int *minimum_height, int *natural_height, int *minimum_baseline, int *natural_baseline) {
                *minimum_height = *natural_height = *minimum_baseline = *natural_baseline = 1;
            }
            static void reckless_progress_bar_get_preferred_size (GtkWidget *widget, GtkRequisition *minimum_size, GtkRequisition *natural_size) {
                minimum_size->width = minimum_size->height = natural_size->width = natural_size->height = 1;
            }
            
            GtkWidget* reckless_progress_bar_new (void) {
                return g_object_new (RECKLESS_PROGRESS_BAR_TYPE, NULL);
            }
        