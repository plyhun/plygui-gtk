#ifndef __RECKLESS_BUTTON_H__
#define __RECKLESS_BUTTON_H__

#ifndef NO_INCLUDE_WITHIN_HEADERS
#include <gtk/gtk.h>
#endif

#define RECKLESS_BUTTON_TYPE                  (reckless_button_get_type ())
#define RECKLESS_BUTTON(obj)                  (G_TYPE_CHECK_INSTANCE_CAST ((obj), RECKLESS_BUTTON_TYPE, RecklessButton))
#define RECKLESS_BUTTON_CLASS(klass)          (G_TYPE_CHECK_CLASS_CAST  ((klass), RECKLESS_BUTTON_TYPE, RecklessButtonClass))
#define IS_RECKLESS_BUTTON(obj)               (G_TYPE_CHECK_INSTANCE_TYPE ((obj), RECKLESS_BUTTON_TYPE))
#define IS_RECKLESS_BUTTON_CLASS(klass)       (G_TYPE_CHECK_CLASS_TYPE  ((klass), RECKLESS_BUTTON_TYPE))
#define RECKLESS_BUTTON_GET_CLASS(obj)        (G_TYPE_INSTANCE_GET_CLASS  ((obj), RECKLESS_BUTTON_TYPE, RecklessButtonClass))

typedef struct _RecklessButton      RecklessButton;
typedef struct _RecklessButtonClass RecklessButtonClass;

struct _RecklessButton
{
    GtkButton container;
};

struct _RecklessButtonClass
{
    GtkButtonClass container_class;
};

GType reckless_button_get_type(void);
GtkWidget* reckless_button_new(void);

static void reckless_button_get_preferred_width(GtkWidget *widget, int *minimal, int *natural);
static void reckless_button_get_preferred_height(GtkWidget *widget, int *minimal, int *natural);

#endif /* __RECKLESS_BUTTON_H__ */
