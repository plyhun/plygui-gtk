
            #ifndef __RECKLESS_TEXT_VIEW_H__
            #define __RECKLESS_TEXT_VIEW_H__
            
            #include <gtk/gtk.h>
            
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
            static void reckless_text_view_get_preferred_height_for_width (GtkWidget *widget, int value, int *minimal, int *natural);
            static void reckless_text_view_get_preferred_width_for_height (GtkWidget *widget, int value, int *minimal, int *natural);
            static void reckless_text_view_get_preferred_height_and_baseline_for_width (GtkWidget *widget, int width, int *minimum_height, int *natural_height, int *minimum_baseline, int *natural_baseline);
            static void reckless_text_view_get_preferred_size (GtkWidget *widget, GtkRequisition *minimum_size, GtkRequisition *natural_size);
            
            #endif /* __RECKLESS_TEXT_VIEW_H__ */        
        