
            #include <gtk/gtk.h>
            #include "reckless_tree_view.h"
            
            G_DEFINE_TYPE( RecklessTreeView, reckless_tree_view, GTK_TYPE_TREE_VIEW )
            
            static void reckless_tree_view_class_init( RecklessTreeViewClass* klass ) {
            	GtkWidgetClass *widget_class = GTK_WIDGET_CLASS(klass);
            	widget_class->get_preferred_width = reckless_tree_view_get_preferred_width;
            	widget_class->get_preferred_height = reckless_tree_view_get_preferred_height;
            }
            static void reckless_tree_view_init( RecklessTreeView* self ) {}
            
            static void reckless_tree_view_get_preferred_width(GtkWidget *widget, int *minimal, int *natural) {
            	*minimal = *natural = 1;
            }
            static void reckless_tree_view_get_preferred_height(GtkWidget *widget, int *minimal, int *natural) {
            	*minimal = *natural = 1;
            }
            GtkWidget* reckless_tree_view_new (void) {
              return g_object_new (RECKLESS_TREE_VIEW_TYPE, NULL);
            }
        