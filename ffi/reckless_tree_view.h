#ifndef __RECKLESS_TREE_VIEW_H__
#define __RECKLESS_TREE_VIEW_H__

#ifndef NO_INCLUDE_WITHIN_HEADERS
#include <gtk/gtk.h>
#endif

#define RECKLESS_TREE_VIEW_TYPE                  (reckless_tree_view_get_type ())
#define RECKLESS_TREE_VIEW(obj)                  (G_TYPE_CHECK_INSTANCE_CAST ((obj), RECKLESS_TREE_VIEW_TYPE, RecklessTreeView))
#define RECKLESS_TREE_VIEW_CLASS(klass)          (G_TYPE_CHECK_CLASS_CAST  ((klass), RECKLESS_TREE_VIEW_TYPE, RecklessTreeViewClass))
#define IS_RECKLESS_TREE_VIEW(obj)               (G_TYPE_CHECK_INSTANCE_TYPE ((obj), RECKLESS_TREE_VIEW_TYPE))
#define IS_RECKLESS_TREE_VIEW_CLASS(klass)       (G_TYPE_CHECK_CLASS_TYPE  ((klass), RECKLESS_TREE_VIEW_TYPE))
#define RECKLESS_TREE_VIEW_GET_CLASS(obj)        (G_TYPE_INSTANCE_GET_CLASS  ((obj), RECKLESS_TREE_VIEW_TYPE, RecklessTreeViewClass))

typedef struct _RecklessTreeView      RecklessTreeView;
typedef struct _RecklessTreeViewClass RecklessTreeViewClass;

struct _RecklessTreeView
{
    GtkTreeView container;
};

struct _RecklessTreeViewClass
{
    GtkTreeViewClass container_class;
};

GType reckless_tree_view_get_type(void);
GtkWidget* reckless_tree_view_new(void);

static void reckless_tree_view_get_preferred_width(GtkWidget *widget, int *minimal, int *natural);
static void reckless_tree_view_get_preferred_height(GtkWidget *widget, int *minimal, int *natural);

#endif /* __RECKLESS_TREE_VIEW_H__ */
