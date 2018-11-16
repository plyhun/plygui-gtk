#include <gtk/gtk.h>
#include "reckless_button.h"

G_DEFINE_TYPE( RecklessButton, reckless_button, GTK_TYPE_BUTTON )

static void reckless_button_class_init( RecklessButtonClass* klass ) {
	GtkWidgetClass *widget_class = GTK_WIDGET_CLASS(klass);
	widget_class->get_preferred_width = reckless_button_get_preferred_width;
	widget_class->get_preferred_height = reckless_button_get_preferred_height;
}
static void reckless_button_init( RecklessButton* self ) {}

static void reckless_button_get_preferred_width(GtkWidget *widget, int *minimal, int *natural) {
	*minimal = *natural = 1;
}
static void reckless_button_get_preferred_height(GtkWidget *widget, int *minimal, int *natural) {
	*minimal = *natural = 1;
}
GtkWidget* reckless_button_new (void) {
  return g_object_new (RECKLESS_BUTTON_TYPE, NULL);
}
