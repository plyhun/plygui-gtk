#include <gtk/gtk.h>
#include "reckless_label.h"

G_DEFINE_TYPE( RecklessLabel, reckless_label, GTK_TYPE_LABEL )

static void reckless_label_class_init( RecklessLabelClass* klass ) {
	GtkWidgetClass *widget_class = GTK_WIDGET_CLASS(klass);
	widget_class->get_preferred_width = reckless_label_get_preferred_width;
	widget_class->get_preferred_height = reckless_label_get_preferred_height;
}
static void reckless_label_init( RecklessLabel* self ) {}

static void reckless_label_get_preferred_width(GtkWidget *widget, int *minimal, int *natural) {
	*minimal = *natural = 1;
}
static void reckless_label_get_preferred_height(GtkWidget *widget, int *minimal, int *natural) {
	*minimal = *natural = 1;
}
GtkWidget* reckless_label_new (void) {
  return g_object_new (RECKLESS_LABEL_TYPE, NULL);
}