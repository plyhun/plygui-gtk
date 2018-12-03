#include <gtk/gtk.h>
#include "reckless_text_view.h"

G_DEFINE_TYPE( RecklessTextView, reckless_text_view, GTK_TYPE_TEXT_VIEW )

static void reckless_text_view_class_init( RecklessTextViewClass* klass ) {
	GtkWidgetClass *widget_class = GTK_WIDGET_CLASS(klass);
	widget_class->get_preferred_width = reckless_text_view_get_preferred_width;
	widget_class->get_preferred_height = reckless_text_view_get_preferred_height;
}
static void reckless_text_view_init( RecklessTextView* self ) {}

static void reckless_text_view_get_preferred_width(GtkWidget *widget, int *minimal, int *natural) {
	*minimal = *natural = 1;
}
static void reckless_text_view_get_preferred_height(GtkWidget *widget, int *minimal, int *natural) {
	*minimal = *natural = 1;
}
GtkWidget* reckless_text_view_new (void) {
  return g_object_new (RECKLESS_TEXT_VIEW_TYPE, NULL);
}
