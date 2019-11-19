
            #ifndef __RECKLESS_PANED_H__
            #define __RECKLESS_PANED_H__
            
            #include <gtk/gtk.h>
            
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
            static void reckless_paned_get_preferred_height_for_width (GtkWidget *widget, int value, int *minimal, int *natural);
            static void reckless_paned_get_preferred_width_for_height (GtkWidget *widget, int value, int *minimal, int *natural);
            static void reckless_paned_get_preferred_height_and_baseline_for_width (GtkWidget *widget, int width, int *minimum_height, int *natural_height, int *minimum_baseline, int *natural_baseline);
            static void reckless_paned_get_preferred_size (GtkWidget *widget, GtkRequisition *minimum_size, GtkRequisition *natural_size);
            
            #endif /* __RECKLESS_PANED_H__ */        
        