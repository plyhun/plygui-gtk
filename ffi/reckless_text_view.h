#ifndef __RECKLESS_TEXT_VIEW_H__
#define __RECKLESS_TEXT_VIEW_H__

#ifndef NO_INCLUDE_WITHIN_HEADERS
#include <gtk/gtk.h>
#endif

#define RECKLESS_TEXT_VIEW_TYPE                  (reckless_text_view_get_type ())
#define RECKLESS_TEXT_VIEW(obj)                  (G_TYPE_CHECK_INSTANCE_CAST ((obj), RECKLESS_TEXT_VIEW_TYPE, RecklessTextView))
#define RECKLESS_TEXT_VIEW_CLASS(klass)          (G_TYPE_CHECK_CLASS_CAST  ((klass), RECKLESS_TEXT_VIEW_TYPE, RecklessTextViewClass))
#define IS_RECKLESS_TEXT_VIEW(obj)               (G_TYPE_CHECK_INSTANCE_TYPE ((obj), RECKLESS_TEXT_VIEW_TYPE))
#define IS_RECKLESS_TEXT_VIEW_CLASS(klass)       (G_TYPE_CHECK_CLASS_TYPE  ((klass), RECKLESS_TEXT_VIEW_TYPE))
#define RECKLESS_TEXT_VIEW_GET_CLASS(obj)        (G_TYPE_INSTANCE_GET_CLASS  ((obj), RECKLESS_TEXT_VIEW_TYPE, RecklessTextViewClass))

typedef struct _RecklessTextView      RecklessTextView;
typedef struct _RecklessTextViewClass RecklessTextViewClass;

struct _RecklessTextView
{
    GtkTextView container;
};

struct _RecklessTextViewClass
{
    GtkTextViewClass container_class;
};

GType reckless_text_view_get_type(void);
GtkWidget* reckless_text_view_new(void);

static void reckless_text_view_get_preferred_width(GtkWidget *widget, int *minimal, int *natural);
static void reckless_text_view_get_preferred_height(GtkWidget *widget, int *minimal, int *natural);

#endif /* __RECKLESS_TEXT_VIEW_H__ */
