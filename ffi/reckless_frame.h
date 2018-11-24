#ifndef __RECKLESS_FRAME_H__
#define __RECKLESS_FRAME_H__

#ifndef NO_INCLUDE_WITHIN_HEADERS
#include <gtk/gtk.h>
#endif

#define RECKLESS_FRAME_TYPE                  (reckless_frame_get_type ())
#define RECKLESS_FRAME(obj)                  (G_TYPE_CHECK_INSTANCE_CAST ((obj), RECKLESS_FRAME_TYPE, RecklessFrame))
#define RECKLESS_FRAME_CLASS(klass)          (G_TYPE_CHECK_CLASS_CAST  ((klass), RECKLESS_FRAME_TYPE, RecklessFrameClass))
#define IS_RECKLESS_FRAME(obj)               (G_TYPE_CHECK_INSTANCE_TYPE ((obj), RECKLESS_FRAME_TYPE))
#define IS_RECKLESS_FRAME_CLASS(klass)       (G_TYPE_CHECK_CLASS_TYPE  ((klass), RECKLESS_FRAME_TYPE))
#define RECKLESS_FRAME_GET_CLASS(obj)        (G_TYPE_INSTANCE_GET_CLASS  ((obj), RECKLESS_FRAME_TYPE, RecklessFrameClass))

typedef struct _RecklessFrame      RecklessFrame;
typedef struct _RecklessFrameClass RecklessFrameClass;

struct _RecklessFrame
{
    GtkFrame container;
};

struct _RecklessFrameClass
{
    GtkFrameClass container_class;
};

GType reckless_frame_get_type(void);
GtkWidget* reckless_frame_new(void);

static void reckless_frame_get_preferred_width(GtkWidget *widget, int *minimal, int *natural);
static void reckless_frame_get_preferred_height(GtkWidget *widget, int *minimal, int *natural);

#endif /* __RECKLESS_FRAME_H__ */
