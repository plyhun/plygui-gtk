use cc;
use heck::*;
use pkg_config;

use std::fs::File;
use std::io::Write;

fn generate(kls: &str, cc_build: &mut cc::Build) {
    let snake = kls.to_owned().to_snake_case();
    let upper_snake = snake.to_uppercase();
    let camel = kls.to_owned().to_camel_case();
    let mut c = File::create(format!("ffi/reckless_{}.c", snake)).unwrap();
    write!(c, r#"
            #include <gtk/gtk.h>
            #include "reckless_{}.h"
            
            G_DEFINE_TYPE( Reckless{}, reckless_{}, GTK_TYPE_{} )
            
            static void reckless_{}_class_init( Reckless{}Class* klass ) {{
            	GtkWidgetClass *widget_class = GTK_WIDGET_CLASS(klass);
            	widget_class->get_preferred_width = reckless_{}_get_preferred_width;
            	widget_class->get_preferred_height = reckless_{}_get_preferred_height;
            }}
            static void reckless_{}_init( Reckless{}* self ) {{}}
            
            static void reckless_{}_get_preferred_width(GtkWidget *widget, int *minimal, int *natural) {{
            	*minimal = *natural = 1;
            }}
            static void reckless_{}_get_preferred_height(GtkWidget *widget, int *minimal, int *natural) {{
            	*minimal = *natural = 1;
            }}
            GtkWidget* reckless_{}_new (void) {{
              return g_object_new (RECKLESS_{}_TYPE, NULL);
            }}
        "#,
        snake.clone(),
        camel.clone(),
        snake.clone(),
        upper_snake.clone(),
        snake.clone(),
        camel.clone(),
        snake.clone(),
        snake.clone(),
        snake.clone(),
        camel.clone(),
        snake.clone(),
        snake.clone(),
        snake.clone(),
        upper_snake.clone(),
    ).unwrap();
    
    let mut h = File::create(format!("ffi/reckless_{}.h", snake)).unwrap();
    write!(h, r#"
            #ifndef __RECKLESS_{}_H__
            #define __RECKLESS_{}_H__
            
            #include <gtk/gtk.h>
            
            #define RECKLESS_{}_TYPE                  (reckless_{}_get_type ())
            #define RECKLESS_{}(obj)                  (G_TYPE_CHECK_INSTANCE_CAST ((obj), RECKLESS_{}_TYPE, Reckless{}))
            #define RECKLESS_{}_CLASS(klass)          (G_TYPE_CHECK_CLASS_CAST  ((klass), RECKLESS_{}_TYPE, Reckless{}Class))
            #define IS_RECKLESS_{}(obj)               (G_TYPE_CHECK_INSTANCE_TYPE ((obj), RECKLESS_{}_TYPE))
            #define IS_RECKLESS_{}_CLASS(klass)       (G_TYPE_CHECK_CLASS_TYPE  ((klass), RECKLESS_{}_TYPE))
            #define RECKLESS_{}_GET_CLASS(obj)        (G_TYPE_INSTANCE_GET_CLASS  ((obj), RECKLESS_{}_TYPE, Reckless{}Class))
            
            typedef struct _Reckless{}      Reckless{};
            typedef struct _Reckless{}Class Reckless{}Class;
            
            struct _Reckless{}
            {{
                Gtk{} container;
            }};
            
            struct _Reckless{}Class
            {{
                Gtk{}Class container_class;
            }};
            
            GType reckless_{}_get_type(void);
            GtkWidget* reckless_{}_new(void);
            
            static void reckless_{}_get_preferred_width(GtkWidget *widget, int *minimal, int *natural);
            static void reckless_{}_get_preferred_height(GtkWidget *widget, int *minimal, int *natural);
            
            #endif /* __RECKLESS_{}_H__ */        
        "#,
        upper_snake.clone(),
        upper_snake.clone(),
        upper_snake.clone(),
        snake.clone(),
        upper_snake.clone(),
        upper_snake.clone(),
        camel.clone(),
        upper_snake.clone(),
        upper_snake.clone(),
        camel.clone(),
        upper_snake.clone(),
        upper_snake.clone(),
        upper_snake.clone(),
        upper_snake.clone(),
        upper_snake.clone(),
        upper_snake.clone(),
        camel.clone(),
        camel.clone(),
        camel.clone(),
        camel.clone(),
        camel.clone(),
        camel.clone(),
        camel.clone(),
        camel.clone(),
        camel.clone(),
        snake.clone(),
        snake.clone(),
        snake.clone(),
        snake.clone(),
        upper_snake.clone(),
    ).unwrap();
    
    let mut rs = File::create(format!("src/reckless/_{}_.rs", snake)).unwrap();
    write!(rs, r#"
            use glib::object::{{Downcast, IsA}};
            use glib::translate::*;
            use gtk::{{ {}, Buildable, Container, Widget }};
            
            use glib_sys as glib_ffi;
            use gobject_sys as gobject_ffi;
            use gtk_sys as gtk_ffi;
            
            use std::{{mem, ptr}};
            
            pub mod ffi {{
                extern "C" {{
                    pub fn reckless_{}_get_type() -> ::glib_sys::GType;
                    pub fn reckless_{}_new() -> *mut ::gtk_sys::GtkWidget;
                }}
            
                #[repr(C)]
                pub struct GtkReckless{}Class {{
                    _truncated_record_marker: ::libc::c_void,
                    // /*Ignored*/field parent_class has incomplete type
                }}
                impl ::std::fmt::Debug for GtkReckless{}Class {{
                    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {{
                        f.debug_struct(&format!("GtkReckless{}Class @ {{:?}}", self as *const _)).finish()
                    }}
                }}
            
                #[repr(C)]
                #[derive(Copy, Clone)]
                pub struct GtkReckless{} {{
                    pub container: ::gtk_sys::Gtk{},
                }}
                impl ::std::fmt::Debug for GtkReckless{} {{
                    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {{
                        f.debug_struct(&format!("GtkReckless{} @ {{:?}}", self as *const _)).field("container", &self.container).finish()
                    }}
                }}
            }}
            
            glib_wrapper! {{
                pub struct Reckless{}(Object<ffi::GtkReckless{}, ffi::GtkReckless{}Class>): [
                     {} => gtk_ffi::Gtk{},
                     Container => gtk_ffi::GtkContainer,
                     Widget => gtk_ffi::GtkWidget,
                     Buildable => gtk_ffi::GtkBuildable,
                ];
            
                match fn {{
                    get_type => || ffi::reckless_{}_get_type(),
                }}
            }}
            
            impl Reckless{} {{
                pub fn new() -> Reckless{} {{
                    if !::gtk::is_initialized_main_thread() {{
                        if ::gtk::is_initialized() {{
                            panic!("GTK may only be used from the main thread.");
                        }} else {{
                            panic!("GTK has not been initialized. Call `gtk::init` first.");
                        }}
                    }}
                    unsafe {{ Widget::from_glib_none(ffi::reckless_{}_new()).downcast_unchecked() }}
                }}
            }}
            impl Default for Reckless{} {{
                fn default() -> Self {{
                    Self::new()
                }}
            }}
            pub trait Reckless{}Ext {{}}
            
            impl<O: IsA<Reckless{}>> Reckless{}Ext for O {{}}    
        "#,
        camel.clone(),
        snake.clone(),
        snake.clone(),
        camel.clone(),
        camel.clone(),
        camel.clone(),
        camel.clone(),
        camel.clone(),
        camel.clone(),
        camel.clone(),
        camel.clone(),
        camel.clone(),
        camel.clone(),
        camel.clone(),
        camel.clone(),
        snake.clone(),
        camel.clone(),
        camel.clone(),
        snake.clone(),
        camel.clone(),
        camel.clone(),
        camel.clone(),
        camel.clone(),
    ).unwrap();
    
    cc_build.file(format!("ffi/reckless_{}.c", snake));
} 

fn main() {
    let gtk_probe = pkg_config::Config::new().atleast_version("3.0").probe("gtk+-3.0").unwrap();
    let glib_probe = pkg_config::Config::new().atleast_version("2.0").probe("glib-2.0").unwrap();

    let mut cc_build = cc::Build::new();

    for lib in gtk_probe.include_paths.as_slice() {
        cc_build.include(lib.to_str().unwrap());
    }
    for lib in glib_probe.include_paths.as_slice() {
        cc_build.include(lib.to_str().unwrap());
    }
    cc_build.include("ffi")
        .define("STATIC_BUILD", None)
        .opt_level(3)
        .warnings(false);
        
        generate("fixed", &mut cc_build);
        generate("button", &mut cc_build);
        generate("paned", &mut cc_build);
        generate("frame", &mut cc_build);
        generate("box", &mut cc_build);
        generate("text_view", &mut cc_build);
        generate("label", &mut cc_build);
        generate("progress_bar", &mut cc_build);
        generate("tree_view", &mut cc_build);
        generate("list_box", &mut cc_build);
        generate("scrolled_window", &mut cc_build);

        cc_build
            .file("ffi/manual/reckless_cell_renderer.c")
            .compile("gtk_reckless");
}
