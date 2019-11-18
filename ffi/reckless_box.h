
            #ifndef __RECKLESS_BOX_H__
            #define __RECKLESS_BOX_H__
            
            #include <gtk/gtk.h>
            
            #define RECKLESS_BOX_TYPE                  (reckless_box_get_type ())
            #define RECKLESS_BOX(obj)                  (G_TYPE_CHECK_INSTANCE_CAST ((obj), RECKLESS_BOX_TYPE, RecklessBox))
            #define RECKLESS_BOX_CLASS(klass)          (G_TYPE_CHECK_CLASS_CAST  ((klass), RECKLESS_BOX_TYPE, RecklessBoxClass))
            #define IS_RECKLESS_BOX(obj)               (G_TYPE_CHECK_INSTANCE_TYPE ((obj), RECKLESS_BOX_TYPE))
            #define IS_RECKLESS_BOX_CLASS(klass)       (G_TYPE_CHECK_CLASS_TYPE  ((klass), RECKLESS_BOX_TYPE))
            #define RECKLESS_BOX_GET_CLASS(obj)        (G_TYPE_INSTANCE_GET_CLASS  ((obj), RECKLESS_BOX_TYPE, RecklessBoxClass))
            
            typedef struct _RecklessBox      RecklessBox;
            typedef struct _RecklessBoxClass RecklessBoxClass;
            
            struct _RecklessBox
            {
                GtkBox container;
            };
            
            struct _RecklessBoxClass
            {
                GtkBoxClass container_class;
            };
            
            GType reckless_box_get_type(void);
            GtkWidget* reckless_box_new(void);
            
            static void reckless_box_get_preferred_width(GtkWidget *widget, int *minimal, int *natural);
            static void reckless_box_get_preferred_height(GtkWidget *widget, int *minimal, int *natural);
            
            #endif /* __RECKLESS_BOX_H__ */        
        