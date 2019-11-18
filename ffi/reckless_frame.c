
            #include <gtk/gtk.h>
            #include "reckless_frame.h"
            
            G_DEFINE_TYPE( RecklessFrame, reckless_frame, GTK_TYPE_FRAME )
            
            static void reckless_frame_class_init( RecklessFrameClass* klass ) {
            	GtkWidgetClass *widget_class = GTK_WIDGET_CLASS(klass);
            	widget_class->get_preferred_width = reckless_frame_get_preferred_width;
            	widget_class->get_preferred_height = reckless_frame_get_preferred_height;
            }
            static void reckless_frame_init( RecklessFrame* self ) {}
            
            static void reckless_frame_get_preferred_width(GtkWidget *widget, int *minimal, int *natural) {
            	*minimal = *natural = 1;
            }
            static void reckless_frame_get_preferred_height(GtkWidget *widget, int *minimal, int *natural) {
            	*minimal = *natural = 1;
            }
            GtkWidget* reckless_frame_new (void) {
              return g_object_new (RECKLESS_FRAME_TYPE, NULL);
            }
        