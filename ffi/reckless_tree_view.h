
            #ifndef __RECKLESS_TREE_VIEW_H__
            #define __RECKLESS_TREE_VIEW_H__
            
            #include <gtk/gtk.h>
            
            #define RECKLESS_TREE_VIEW_TYPE                  (reckless_tree_view_get_type ())
            #define RECKLESS_TREE_VIEW(obj)                  (G_TYPE_CHECK_INSTANCE_CAST ((obj), RECKLESS_TREE_VIEW_TYPE, RecklessTreeView))
            #define RECKLESS_TREE_VIEW_CLASS(klass)          (G_TYPE_CHECK_CLASS_CAST  ((klass), RECKLESS_TREE_VIEW_TYPE, RecklessTreeViewClass))
            #define IS_RECKLESS_TREE_VIEW(obj)               (G_TYPE_CHECK_INSTANCE_TYPE ((obj), RECKLESS_TREE_VIEW_TYPE))
            #define IS_RECKLESS_TREE_VIEW_CLASS(klass)       (G_TYPE_CHECK_CLASS_TYPE  ((klass), RECKLESS_TREE_VIEW_TYPE))
            #define RECKLESS_TREE_VIEW_GET_CLASS(obj)        (G_TYPE_INSTANCE_GET_CLASS  ((obj), RECKLESS_TREE_VIEW_TYPE, RecklessTreeViewClass))
            
            typedef struct _RecklessTreeView      RecklessTreeView;
            typedef struct _RecklessTreeViewClass RecklessTreeViewClass;
            
            struct _RecklessTreeView
            {
                GtkTreeView container;
            };
            
            struct _RecklessTreeViewClass
            {
                GtkTreeViewClass container_class;
            };
            
            GType reckless_tree_view_get_type(void);
            GtkWidget* reckless_tree_view_new(void);
            
            static void reckless_tree_view_get_preferred_width(GtkWidget *widget, int *minimal, int *natural);
            static void reckless_tree_view_get_preferred_height(GtkWidget *widget, int *minimal, int *natural);
            static void reckless_tree_view_get_preferred_height_for_width (GtkWidget *widget, int value, int *minimal, int *natural);
            static void reckless_tree_view_get_preferred_width_for_height (GtkWidget *widget, int value, int *minimal, int *natural);
            static void reckless_tree_view_get_preferred_height_and_baseline_for_width (GtkWidget *widget, int width, int *minimum_height, int *natural_height, int *minimum_baseline, int *natural_baseline);
            static void reckless_tree_view_get_preferred_size (GtkWidget *widget, GtkRequisition *minimum_size, GtkRequisition *natural_size);
            
            #endif /* __RECKLESS_TREE_VIEW_H__ */        
        