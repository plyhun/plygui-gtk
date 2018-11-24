#ifndef __RECKLESS_PANED_H__
#define __RECKLESS_PANED_H__

#ifndef NO_INCLUDE_WITHIN_HEADERS
#include <gtk/gtk.h>
#endif

#define RECKLESS_PANED_TYPE                  (reckless_paned_get_type ())
#define RECKLESS_PANED(obj)                  (G_TYPE_CHECK_INSTANCE_CAST ((obj), RECKLESS_PANED_TYPE, RecklessPaned))
#define RECKLESS_PANED_CLASS(klass)          (G_TYPE_CHECK_CLASS_CAST  ((klass), RECKLESS_PANED_TYPE, RecklessPanedClass))
#define IS_RECKLESS_PANED(obj)               (G_TYPE_CHECK_INSTANCE_TYPE ((obj), RECKLESS_PANED_TYPE))
#define IS_RECKLESS_PANED_CLASS(klass)       (G_TYPE_CHECK_CLASS_TYPE  ((klass), RECKLESS_PANED_TYPE))
#define RECKLESS_PANED_GET_CLASS(obj)        (G_TYPE_INSTANCE_GET_CLASS  ((obj), RECKLESS_PANED_TYPE, RecklessPanedClass))

typedef struct _RecklessPaned      RecklessPaned;
typedef struct _RecklessPanedClass RecklessPanedClass;

struct _RecklessPaned
{
    GtkPaned container;
};

struct _RecklessPanedClass
{
    GtkPanedClass container_class;
};

GType reckless_paned_get_type(void);
GtkWidget* reckless_paned_new(void);

static void reckless_paned_get_preferred_width(GtkWidget *widget, int *minimal, int *natural);
static void reckless_paned_get_preferred_height(GtkWidget *widget, int *minimal, int *natural);

#endif /* __RECKLESS_PANED_H__ */
