#include <gtk/gtk.h>

#define TYPE_RECKLESS_CELL_RENDERER             (reckless_cell_renderer_get_type())
#define RECKLESS_CELL_RENDERER(obj)             (G_TYPE_CHECK_INSTANCE_CAST((obj),  TYPE_RECKLESS_CELL_RENDERER, RecklessCellRenderer))
#define RECKLESS_CELL_RENDERER_CLASS(clz)     	(G_TYPE_CHECK_CLASS_CAST ((clz),  TYPE_RECKLESS_CELL_RENDERER, RecklessCellRendererClass))
#define IS_CELL_PROGRESS_PROGRESS(obj)          (G_TYPE_CHECK_INSTANCE_TYPE ((obj), TYPE_RECKLESS_CELL_RENDERER))
#define IS_CELL_PROGRESS_PROGRESS_CLASS(clz)  	(G_TYPE_CHECK_CLASS_TYPE ((clz),  TYPE_RECKLESS_CELL_RENDERER))
#define RECKLESS_CELL_RENDERER_GET_CLASS(obj)   (G_TYPE_INSTANCE_GET_CLASS ((obj),  TYPE_RECKLESS_CELL_RENDERER, RecklessCellRendererClass))

typedef struct _RecklessCellRenderer RecklessCellRenderer;
typedef struct _RecklessCellRendererClass RecklessCellRendererClass;

struct _RecklessCellRenderer {
	GtkCellRenderer parent;
	GtkWidget *cell;
};

struct _RecklessCellRendererClass {
	GtkCellRendererClass parent_class;
};

GType reckless_cell_renderer_get_type(void);

GObject* reckless_cell_renderer_new(void);
