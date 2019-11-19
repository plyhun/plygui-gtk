
            #ifndef __RECKLESS_LABEL_H__
            #define __RECKLESS_LABEL_H__
            
            #include <gtk/gtk.h>
            
            #define RECKLESS_LABEL_TYPE                  (reckless_label_get_type ())
            #define RECKLESS_LABEL(obj)                  (G_TYPE_CHECK_INSTANCE_CAST ((obj), RECKLESS_LABEL_TYPE, RecklessLabel))
            #define RECKLESS_LABEL_CLASS(klass)          (G_TYPE_CHECK_CLASS_CAST  ((klass), RECKLESS_LABEL_TYPE, RecklessLabelClass))
            #define IS_RECKLESS_LABEL(obj)               (G_TYPE_CHECK_INSTANCE_TYPE ((obj), RECKLESS_LABEL_TYPE))
            #define IS_RECKLESS_LABEL_CLASS(klass)       (G_TYPE_CHECK_CLASS_TYPE  ((klass), RECKLESS_LABEL_TYPE))
            #define RECKLESS_LABEL_GET_CLASS(obj)        (G_TYPE_INSTANCE_GET_CLASS  ((obj), RECKLESS_LABEL_TYPE, RecklessLabelClass))
            
            typedef struct _RecklessLabel      RecklessLabel;
            typedef struct _RecklessLabelClass RecklessLabelClass;
            
            struct _RecklessLabel
            {
                GtkLabel container;
            };
            
            struct _RecklessLabelClass
            {
                GtkLabelClass container_class;
            };
            
            GType reckless_label_get_type(void);
            GtkWidget* reckless_label_new(void);
            
            static void reckless_label_get_preferred_width(GtkWidget *widget, int *minimal, int *natural);
            static void reckless_label_get_preferred_height(GtkWidget *widget, int *minimal, int *natural);
            static void reckless_label_get_preferred_height_for_width (GtkWidget *widget, int value, int *minimal, int *natural);
            static void reckless_label_get_preferred_width_for_height (GtkWidget *widget, int value, int *minimal, int *natural);
            static void reckless_label_get_preferred_height_and_baseline_for_width (GtkWidget *widget, int width, int *minimum_height, int *natural_height, int *minimum_baseline, int *natural_baseline);
            static void reckless_label_get_preferred_size (GtkWidget *widget, GtkRequisition *minimum_size, GtkRequisition *natural_size);
            
            #endif /* __RECKLESS_LABEL_H__ */        
        