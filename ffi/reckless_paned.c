
            #include <gtk/gtk.h>
            #include "reckless_paned.h"
            
            G_DEFINE_TYPE( RecklessPaned, reckless_paned, GTK_TYPE_PANED )
            
            static void reckless_paned_class_init( RecklessPanedClass* klass ) {
            	GtkWidgetClass *widget_class = GTK_WIDGET_CLASS(klass);
            	widget_class->get_preferred_width = reckless_paned_get_preferred_width;
            	widget_class->get_preferred_height = reckless_paned_get_preferred_height;
            }
            static void reckless_paned_init( RecklessPaned* self ) {}
            
            static void reckless_paned_get_preferred_width(GtkWidget *widget, int *minimal, int *natural) {
            	*minimal = *natural = 1;
            }
            static void reckless_paned_get_preferred_height(GtkWidget *widget, int *minimal, int *natural) {
            	*minimal = *natural = 1;
            }
            GtkWidget* reckless_paned_new (void) {
              return g_object_new (RECKLESS_PANED_TYPE, NULL);
            }
        