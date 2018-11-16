#include <gtk/gtk.h>
#include "reckless_box.h"

G_DEFINE_TYPE( RecklessBox, reckless_box, GTK_TYPE_BOX )

static void reckless_box_class_init( RecklessBoxClass* klass ) {
	GtkWidgetClass *widget_class = GTK_WIDGET_CLASS(klass);
	widget_class->get_preferred_width = reckless_box_get_preferred_width;
	widget_class->get_preferred_height = reckless_box_get_preferred_height;
}
static void reckless_box_init( RecklessBox* self ) {}

static void reckless_box_get_preferred_width(GtkWidget *widget, int *minimal, int *natural) {
	*minimal = *natural = 1;
}
static void reckless_box_get_preferred_height(GtkWidget *widget, int *minimal, int *natural) {
	*minimal = *natural = 1;
}
GtkWidget* reckless_box_new (void) {
  return g_object_new (RECKLESS_BOX_TYPE, NULL);
}
