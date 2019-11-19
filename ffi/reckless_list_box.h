
            #ifndef __RECKLESS_LIST_BOX_H__
            #define __RECKLESS_LIST_BOX_H__
            
            #include <gtk/gtk.h>
            
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
            static void reckless_list_box_get_preferred_height_for_width (GtkWidget *widget, int value, int *minimal, int *natural);
            static void reckless_list_box_get_preferred_width_for_height (GtkWidget *widget, int value, int *minimal, int *natural);
            static void reckless_list_box_get_preferred_height_and_baseline_for_width (GtkWidget *widget, int width, int *minimum_height, int *natural_height, int *minimum_baseline, int *natural_baseline);
            static void reckless_list_box_get_preferred_size (GtkWidget *widget, GtkRequisition *minimum_size, GtkRequisition *natural_size);
            
            #endif /* __RECKLESS_LIST_BOX_H__ */        
        