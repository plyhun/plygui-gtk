#include "reckless_cell_renderer.h"

static void reckless_cell_renderer_init(RecklessCellRenderer *cell);

static void reckless_cell_renderer_class_init(RecklessCellRendererClass *clz);

static void reckless_cell_renderer_get_property(GObject *object, guint param_id,
		GValue *value, GParamSpec *pspec);

static void reckless_cell_renderer_set_property(GObject *object, guint param_id,
		const GValue *value, GParamSpec *pspec);

static void reckless_cell_renderer_finalize(GObject *gobject);

static void reckless_cell_renderer_get_size(GtkCellRenderer *cell,
		GtkWidget *widget, const GdkRectangle *cell_area, gint *x_offset,
		gint *y_offset, gint *width, gint *height);

static void reckless_cell_renderer_render(GtkCellRenderer *cell, cairo_t *ctx,
		GtkWidget *widget, const GdkRectangle *background_area,
		const GdkRectangle *cell_area, GtkCellRendererState state);

static void reckless_cell_renderer_get_preferred_width           (GtkCellRenderer         *cell,
                                                                  GtkWidget               *widget,
                                                                  gint                    *minimum_size,
                                                                  gint                    *natural_size);
static void reckless_cell_renderer_get_preferred_height          (GtkCellRenderer         *cell,
                                                                  GtkWidget               *widget,
                                                                  gint                    *minimum_size,
                                                                  gint                    *natural_size);
static void reckless_cell_renderer_get_preferred_height_for_width(GtkCellRenderer         *cell,
                                                                  GtkWidget               *widget,
                                                                  gint                     width,
                                                                  gint                    *minimum_height,
                                                                  gint                    *natural_height);
static void reckless_cell_renderer_get_preferred_width_for_height(GtkCellRenderer         *cell,
                                                                  GtkWidget               *widget,
                                                                  gint                     height,
                                                                  gint                    *minimum_width,
                                                                  gint                    *natural_width);

enum {
	PROP_CELL = 1,
};
static gpointer parent_class;

GType reckless_cell_renderer_get_type(void) {
	static GType cell__type = 0;

	if (cell__type)
		return cell__type;

	if (1) {
		static const GTypeInfo cell__info = { sizeof(RecklessCellRendererClass),
				NULL,
				NULL,
				(GClassInitFunc) reckless_cell_renderer_class_init, NULL,
				NULL,
				sizeof(RecklessCellRenderer), 0,
				(GInstanceInitFunc) reckless_cell_renderer_init, };

		cell__type = g_type_register_static(GTK_TYPE_CELL_RENDERER,
				"RecklessCellRenderer", &cell__info, 0);
	}

	return cell__type;
}

static void reckless_cell_renderer_init(RecklessCellRenderer *cellrenderer) {
}

static void reckless_cell_renderer_class_init(RecklessCellRendererClass *clz) {
	GtkCellRendererClass *cell_class = GTK_CELL_RENDERER_CLASS(clz);
	GObjectClass *object_class = G_OBJECT_CLASS(clz);

	parent_class = g_type_class_peek_parent(clz);
	object_class->finalize = reckless_cell_renderer_finalize;

	object_class->get_property = reckless_cell_renderer_get_property;
	object_class->set_property = reckless_cell_renderer_set_property;

	cell_class->get_size = reckless_cell_renderer_get_size;
	cell_class->render = reckless_cell_renderer_render;
	cell_class->get_preferred_width            = reckless_cell_renderer_get_preferred_width;
	cell_class->get_preferred_height           = reckless_cell_renderer_get_preferred_height;
	cell_class->get_preferred_width_for_height = reckless_cell_renderer_get_preferred_width_for_height;
	cell_class->get_preferred_height_for_width = reckless_cell_renderer_get_preferred_height_for_width;

	g_object_class_install_property(object_class, PROP_CELL,
	g_param_spec_pointer("cell", "Cell", "Widget to display", G_PARAM_READWRITE));
}


static void reckless_cell_renderer_finalize(GObject *object) {
	/*
	 RecklessCellRenderer *cellrenderer = RECKLESS_CELL_RENDERER(object);
	 */

	(*G_OBJECT_CLASS(parent_class)->finalize)(object);
}

static void reckless_cell_renderer_get_property(GObject *object, guint param_id,
		GValue *value, GParamSpec *psec) {
	RecklessCellRenderer *cell = RECKLESS_CELL_RENDERER(object);

	switch (param_id) {
	case PROP_CELL:
		g_value_set_pointer(value, cell->cell);
		break;

	default:
		G_OBJECT_WARN_INVALID_PROPERTY_ID(object, param_id, psec);
		break;
	}
}

static void reckless_cell_renderer_set_property(GObject *object, guint param_id,
		const GValue *value, GParamSpec *pspec) {
	RecklessCellRenderer *cell = RECKLESS_CELL_RENDERER(object);

	switch (param_id) {
	case PROP_CELL:
		cell->cell = g_value_get_pointer(value);
		break;

	default:
		G_OBJECT_WARN_INVALID_PROPERTY_ID(object, param_id, pspec);
		break;
	}
}

GObject*
reckless_cell_renderer_new(void) {
	return g_object_new(TYPE_RECKLESS_CELL_RENDERER, NULL);
}

static void
reckless_cell_renderer_get_preferred_width (GtkCellRenderer *cell,
                                            GtkWidget       *widget,
                                            gint            *minimum,
                                            gint            *natural)
{
	if (minimum) *minimum = 1;
	if (natural) *natural = 1;
}

static void
reckless_cell_renderer_get_preferred_height (GtkCellRenderer *cell,
                                             GtkWidget       *widget,
                                             gint            *minimum,
                                             gint            *natural)
{
	if (minimum) *minimum = 1;
	if (natural) *natural = 1;
}


static void
reckless_cell_renderer_get_preferred_height_for_width (GtkCellRenderer *cell,
                                                       GtkWidget       *widget,
                                                       gint             width,
                                                       gint            *minimum,
                                                       gint            *natural)
{
	if (minimum) *minimum = 1;
	if (natural) *natural = 1;
}

static void
reckless_cell_renderer_get_preferred_width_for_height (GtkCellRenderer *cell,
                                                       GtkWidget       *widget,
                                                       gint             height,
                                                       gint            *minimum,
                                                       gint            *natural)
{
	if (minimum) *minimum = 1;
	if (natural) *natural = 1;
}

static void reckless_cell_renderer_get_size(GtkCellRenderer *cell,
		GtkWidget *widget, const GdkRectangle *cell_area, gint *x_offset,
		gint *y_offset, gint *width, gint *height) {
	gint calc_width;
	gint calc_height;

	RecklessCellRenderer *rc = RECKLESS_CELL_RENDERER(cell);
	gtk_widget_get_size_request(rc->cell, &calc_width, &calc_height);

	if (width) {
		*width = calc_width;
	}
	if (height) {
		*height = calc_height;
	}
	if (cell_area) {
		if (x_offset) {
			*x_offset = (cell_area->width - *width);
			*x_offset = MAX(*x_offset, 0);
		}
		if (y_offset) {
			*y_offset = (cell_area->height - *height);
			*y_offset = MAX(*y_offset, 0);
		}
	}
}
static void reckless_cell_renderer_render(GtkCellRenderer *cell, cairo_t *ctx,
		GtkWidget *widget, const GdkRectangle *background_area,
		const GdkRectangle *cell_area, GtkCellRendererState state) {
	GdkRectangle allo;
	gint calc_width;
	gint calc_height;

	cairo_save(ctx);

	RecklessCellRenderer *rc = RECKLESS_CELL_RENDERER(cell);
	gtk_widget_get_size_request(rc->cell, &calc_width, &calc_height);

	allo.x = MAX(cell_area->x, 0);
	allo.y = MAX(cell_area->y, 0);
	allo.width = calc_width;
	allo.height = calc_height;

	gtk_widget_size_allocate(rc->cell, &allo);
	cairo_translate(ctx, allo.x, allo.y);
	gtk_widget_draw(rc->cell, ctx);

	cairo_restore(ctx);
}
