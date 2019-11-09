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

static void reckless_cell_renderer_render(GtkCellRenderer *cell,
		cairo_t *ctx, GtkWidget *widget, const GdkRectangle *background_area,
		const GdkRectangle *cell_area, GtkCellRendererState state);

enum
{
  PROP_CELL = 1,
};
static gpointer parent_class;

GType reckless_cell_renderer_get_type(void) {
	static GType cell__type = 0;

	if (cell__type)
		return cell__type;

	if (1) {
		static const GTypeInfo cell__info = {
				sizeof(RecklessCellRendererClass), NULL, /* base_init */
				NULL, /* base_finalize */
				(GClassInitFunc) reckless_cell_renderer_class_init, NULL, /* class_finalize */
				NULL, /* class_data */
				sizeof(RecklessCellRenderer), 0, /* n_preallocs */
				(GInstanceInitFunc) reckless_cell_renderer_init, };

		/* Derive from GtkCellRenderer */
		cell__type = g_type_register_static(GTK_TYPE_CELL_RENDERER,
				"RecklessCellRenderer", &cell__info, 0);
	}

	return cell__type;
}

static void reckless_cell_renderer_init(
		RecklessCellRenderer *cellrenderer) {
//	GTK_CELL_RENDERER(cellrenderer)->mode = GTK_CELL_RENDERER_MODE_INERT;
//	GTK_CELL_RENDERER(cellrenderer)->xpad = 2;
//	GTK_CELL_RENDERER(cellrenderer)->ypad = 2;
}

static void reckless_cell_renderer_class_init(RecklessCellRendererClass *clz) {
	GtkCellRendererClass *cell_class = GTK_CELL_RENDERER_CLASS(clz);
	GObjectClass *object_class = G_OBJECT_CLASS(clz);

	parent_class = g_type_class_peek_parent(clz);
	object_class->finalize = reckless_cell_renderer_finalize;

	/* Hook up functions to set and get our
	 *   custom cell renderer properties */
	object_class->get_property = reckless_cell_renderer_get_property;
	object_class->set_property = reckless_cell_renderer_set_property;

	/* Override the two crucial functions that are the heart
	 *   of a cell renderer in the parent class */
	cell_class->get_size = reckless_cell_renderer_get_size;
	cell_class->render = reckless_cell_renderer_render;

	/* Install our very own properties */
	g_object_class_install_property(object_class, PROP_CELL,
			g_param_spec_double("percentage", "Percentage",
					"The fractional  to display", 0, 1, 0,
					G_PARAM_READWRITE));
}

/***************************************************************************
 *
 *  reckless_cell_renderer_finalize: free any resources here
 *
 ***************************************************************************/

static void reckless_cell_renderer_finalize(GObject *object) {
	/*
	 RecklessCellRenderer *cellrenderer = RECKLESS_CELL_RENDERER(object);
	 */

	/* Free any dynamically allocated resources here */

	(*G_OBJECT_CLASS(parent_class)->finalize)(object);
}

/***************************************************************************
 *
 *  reckless_cell_renderer_get_property: as it says
 *
 ***************************************************************************/

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

/***************************************************************************
 *
 *  reckless_cell_renderer_set_property: as it says
 *
 ***************************************************************************/

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

/***************************************************************************
 *
 *  reckless_cell_renderer_new: return a new cell renderer instance
 *
 ***************************************************************************/

GtkCellRenderer*
reckless_cell_renderer_new(void) {
	return g_object_new(TYPE_RECKLESS_CELL_RENDERER, NULL);
}

static void reckless_cell_renderer_get_size(GtkCellRenderer *cell,
		GtkWidget *widget, const GdkRectangle *cell_area, gint *x_offset,
		gint *y_offset, gint *width, gint *height) {

	RecklessCellRenderer *rc = RECKLESS_CELL_RENDERER (cell);
	gtk_widget_get_size_request(rc->cell, width, height);

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
static void reckless_cell_renderer_render(GtkCellRenderer *cell,
		cairo_t *ctx,
		GtkWidget *widget,
		const GdkRectangle *background_area,
		const GdkRectangle *cell_area,
		GtkCellRendererState state)
{
	RecklessCellRenderer *rc = RECKLESS_CELL_RENDERER (cell);
	GtkStateType ty;
	gint width, height;
	gint x_offset, y_offset;

	reckless_cell_renderer_get_size (cell, widget, cell_area,
			&x_offset, &y_offset,
			&width, &height);

	/*if (GTK_WIDGET_HAS_FOCUS (widget))
	state = GTK_STATE_ACTIVE;
	else
	state = GTK_STATE_NORMAL;

	width -= cell->xpad*2;
	height -= cell->ypad*2;

	gtk_paint_box (widget->style,
			window,
			GTK_STATE_NORMAL, GTK_SHADOW_IN,
			NULL, widget, "trough",
			cell_area->x + x_offset + cell->xpad,
			cell_area->y + y_offset + cell->ypad,
			width - 1, height - 1);

	gtk_paint_box (widget->style,
			window,
			state, GTK_SHADOW_OUT,
			NULL, widget, "bar",
			cell_area->x + x_offset + cell->xpad,
			cell_area->y + y_offset + cell->ypad,
			width * cell->,
			height - 1);*/
}
