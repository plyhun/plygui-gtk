
            #ifndef __RECKLESS_SCROLLED_WINDOW_H__
            #define __RECKLESS_SCROLLED_WINDOW_H__
            
            #include <gtk/gtk.h>
            
            #define RECKLESS_SCROLLED_WINDOW_TYPE                  (reckless_scrolled_window_get_type ())
            #define RECKLESS_SCROLLED_WINDOW(obj)                  (G_TYPE_CHECK_INSTANCE_CAST ((obj), RECKLESS_SCROLLED_WINDOW_TYPE, RecklessScrolledWindow))
            #define RECKLESS_SCROLLED_WINDOW_CLASS(klass)          (G_TYPE_CHECK_CLASS_CAST  ((klass), RECKLESS_SCROLLED_WINDOW_TYPE, RecklessScrolledWindowClass))
            #define IS_RECKLESS_SCROLLED_WINDOW(obj)               (G_TYPE_CHECK_INSTANCE_TYPE ((obj), RECKLESS_SCROLLED_WINDOW_TYPE))
            #define IS_RECKLESS_SCROLLED_WINDOW_CLASS(klass)       (G_TYPE_CHECK_CLASS_TYPE  ((klass), RECKLESS_SCROLLED_WINDOW_TYPE))
            #define RECKLESS_SCROLLED_WINDOW_GET_CLASS(obj)        (G_TYPE_INSTANCE_GET_CLASS  ((obj), RECKLESS_SCROLLED_WINDOW_TYPE, RecklessScrolledWindowClass))
            
            typedef struct _RecklessScrolledWindow      RecklessScrolledWindow;
            typedef struct _RecklessScrolledWindowClass RecklessScrolledWindowClass;
            
            struct _RecklessScrolledWindow
            {
                GtkScrolledWindow container;
            };
            
            struct _RecklessScrolledWindowClass
            {
                GtkScrolledWindowClass container_class;
            };
            
            GType reckless_scrolled_window_get_type(void);
            GtkWidget* reckless_scrolled_window_new(void);
            
            static void reckless_scrolled_window_get_preferred_width(GtkWidget *widget, int *minimal, int *natural);
            static void reckless_scrolled_window_get_preferred_height(GtkWidget *widget, int *minimal, int *natural);
            static void reckless_scrolled_window_get_preferred_height_for_width (GtkWidget *widget, int value, int *minimal, int *natural);
            static void reckless_scrolled_window_get_preferred_width_for_height (GtkWidget *widget, int value, int *minimal, int *natural);
            static void reckless_scrolled_window_get_preferred_height_and_baseline_for_width (GtkWidget *widget, int width, int *minimum_height, int *natural_height, int *minimum_baseline, int *natural_baseline);
            static void reckless_scrolled_window_get_preferred_size (GtkWidget *widget, GtkRequisition *minimum_size, GtkRequisition *natural_size);
            
            #endif /* __RECKLESS_SCROLLED_WINDOW_H__ */        
        