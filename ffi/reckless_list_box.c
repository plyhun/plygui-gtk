#include <gtk/gtk.h>
#include "reckless_list_box.h"

G_DEFINE_TYPE( RecklessListBox, reckless_list_box, GTK_TYPE_LIST_BOX )

static void reckless_list_box_class_init( RecklessListBoxClass* klass ) {
	GtkWidgetClass *widget_class = GTK_WIDGET_CLASS(klass);
	widget_class->get_preferred_width = reckless_list_box_get_preferred_width;
	widget_class->get_preferred_height = reckless_list_box_get_preferred_height;
}
static void reckless_list_box_init( RecklessListBox* self ) {}

static void reckless_list_box_get_preferred_width(GtkWidget *widget, int *minimal, int *natural) {
	*minimal = *natural = 1;
}
static void reckless_list_box_get_preferred_height(GtkWidget *widget, int *minimal, int *natural) {
	*minimal = *natural = 1;
}
GtkWidget* reckless_list_box_new (void) {
  return g_object_new (RECKLESS_LIST_BOX_TYPE, NULL);
}
