
            #include <gtk/gtk.h>
            #include "reckless_button.h"
            
            G_DEFINE_TYPE( RecklessButton, reckless_button, GTK_TYPE_BUTTON )
            
            static void reckless_button_class_init( RecklessButtonClass* klass ) {
            	GtkWidgetClass *widget_class = GTK_WIDGET_CLASS(klass);
            	widget_class->get_preferred_width = reckless_button_get_preferred_width;
            	widget_class->get_preferred_height = reckless_button_get_preferred_height;
            	widget_class->get_preferred_height_for_width = reckless_button_get_preferred_height_for_width;
                widget_class->get_preferred_width_for_height = reckless_button_get_preferred_width_for_height;
                widget_class->get_preferred_height_and_baseline_for_width = reckless_button_get_preferred_height_and_baseline_for_width;
                // widget_class->get_preferred_size = reckless_button_get_preferred_size;
            }
            static void reckless_button_init( RecklessButton* self ) {}
            
            static void reckless_button_get_preferred_width(GtkWidget *widget, int *minimal, int *natural) {
            	*minimal = *natural = 1;
            }
            static void reckless_button_get_preferred_height(GtkWidget *widget, int *minimal, int *natural) {
            	*minimal = *natural = 1;
            }
            
            static void reckless_button_get_preferred_height_for_width (GtkWidget *widget, int value, int *minimal, int *natural) {
                *minimal = *natural = 1;
            }
            static void reckless_button_get_preferred_width_for_height (GtkWidget *widget, int value, int *minimal, int *natural) {
                *minimal = *natural = 1;
            }
            static void reckless_button_get_preferred_height_and_baseline_for_width (GtkWidget *widget, int width, int *minimum_height, int *natural_height, int *minimum_baseline, int *natural_baseline) {
                *minimum_height = *natural_height = *minimum_baseline = *natural_baseline = 1;
            }
            static void reckless_button_get_preferred_size (GtkWidget *widget, GtkRequisition *minimum_size, GtkRequisition *natural_size) {
                minimum_size->width = minimum_size->height = natural_size->width = natural_size->height = 1;
            }
            
            GtkWidget* reckless_button_new (void) {
                return g_object_new (RECKLESS_BUTTON_TYPE, NULL);
            }
        