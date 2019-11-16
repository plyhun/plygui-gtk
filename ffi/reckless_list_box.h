#ifndef __RECKLESS_LIST_BOX_H__
#define __RECKLESS_LIST_BOX_H__

#ifndef NO_INCLUDE_WITHIN_HEADERS
#include <gtk/gtk.h>
#endif

#define RECKLESS_LIST_BOX_TYPE                  (reckless_list_box_get_type ())
#define RECKLESS_LIST_BOX(obj)                  (G_TYPE_CHECK_INSTANCE_CAST ((obj), RECKLESS_LIST_BOX_TYPE, RecklessListBox))
#define RECKLESS_LIST_BOX_CLASS(klass)          (G_TYPE_CHECK_CLASS_CAST  ((klass), RECKLESS_LIST_BOX_TYPE, RecklessListBoxClass))
#define IS_RECKLESS_LIST_BOX(obj)               (G_TYPE_CHECK_INSTANCE_TYPE ((obj), RECKLESS_LIST_BOX_TYPE))
#define IS_RECKLESS_LIST_BOX_CLASS(klass)       (G_TYPE_CHECK_CLASS_TYPE  ((klass), RECKLESS_LIST_BOX_TYPE))
#define RECKLESS_LIST_BOX_GET_CLASS(obj)        (G_TYPE_INSTANCE_GET_CLASS  ((obj), RECKLESS_LIST_BOX_TYPE, RecklessListBoxClass))

typedef struct _RecklessListBox      RecklessListBox;
typedef struct _RecklessListBoxClass RecklessListBoxClass;

struct _RecklessListBox
{
    GtkListBox container;
};

struct _RecklessListBoxClass
{
    GtkListBoxClass container_class;
};

GType reckless_list_box_get_type(void);
GtkWidget* reckless_list_box_new(void);

static void reckless_list_box_get_preferred_width(GtkWidget *widget, int *minimal, int *natural);
static void reckless_list_box_get_preferred_height(GtkWidget *widget, int *minimal, int *natural);

#endif /* __RECKLESS_LIST_BOX_H__ */
