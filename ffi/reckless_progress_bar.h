
            #ifndef __RECKLESS_PROGRESS_BAR_H__
            #define __RECKLESS_PROGRESS_BAR_H__
            
            #include <gtk/gtk.h>
            
            #define RECKLESS_PROGRESS_BAR_TYPE                  (reckless_progress_bar_get_type ())
            #define RECKLESS_PROGRESS_BAR(obj)                  (G_TYPE_CHECK_INSTANCE_CAST ((obj), RECKLESS_PROGRESS_BAR_TYPE, RecklessProgressBar))
            #define RECKLESS_PROGRESS_BAR_CLASS(klass)          (G_TYPE_CHECK_CLASS_CAST  ((klass), RECKLESS_PROGRESS_BAR_TYPE, RecklessProgressBarClass))
            #define IS_RECKLESS_PROGRESS_BAR(obj)               (G_TYPE_CHECK_INSTANCE_TYPE ((obj), RECKLESS_PROGRESS_BAR_TYPE))
            #define IS_RECKLESS_PROGRESS_BAR_CLASS(klass)       (G_TYPE_CHECK_CLASS_TYPE  ((klass), RECKLESS_PROGRESS_BAR_TYPE))
            #define RECKLESS_PROGRESS_BAR_GET_CLASS(obj)        (G_TYPE_INSTANCE_GET_CLASS  ((obj), RECKLESS_PROGRESS_BAR_TYPE, RecklessProgressBarClass))
            
            typedef struct _RecklessProgressBar      RecklessProgressBar;
            typedef struct _RecklessProgressBarClass RecklessProgressBarClass;
            
            struct _RecklessProgressBar
            {
                GtkProgressBar container;
            };
            
            struct _RecklessProgressBarClass
            {
                GtkProgressBarClass container_class;
            };
            
            GType reckless_progress_bar_get_type(void);
            GtkWidget* reckless_progress_bar_new(void);
            
            static void reckless_progress_bar_get_preferred_width(GtkWidget *widget, int *minimal, int *natural);
            static void reckless_progress_bar_get_preferred_height(GtkWidget *widget, int *minimal, int *natural);
            
            #endif /* __RECKLESS_PROGRESS_BAR_H__ */        
        