use glib::{Object, Cast};
use glib::object::IsA;
use glib::translate::*;
use gtk::CellRenderer;

pub mod ffi {
    extern "C" {
        pub fn reckless_cell_renderer_get_type() -> ::glib_sys::GType;
        pub fn reckless_cell_renderer_new() -> *mut ::gobject_sys::GObject;
    }

    #[repr(C)]
    pub struct GtkRecklessCellRendererClass {
        _truncated_record_marker: ::libc::c_void,
        // /*Ignored*/field parent_class has incomplete type
    }
    impl ::std::fmt::Debug for GtkRecklessCellRendererClass {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.debug_struct(&format!("GtkRecklessCellRendererClass @ {:?}", self as *const _)).finish()
        }
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct GtkRecklessCellRenderer {
        pub container: ::gtk_sys::GtkCellRenderer,
    }
    impl ::std::fmt::Debug for GtkRecklessCellRenderer {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.debug_struct(&format!("GtkRecklessCellRenderer @ {:?}", self as *const _)).field("container", &self.container).finish()
        }
    }
}

wrapper! {
    pub struct RecklessCellRenderer(Object<ffi::GtkRecklessCellRenderer, ffi::GtkRecklessCellRendererClass>) @implements CellRenderer;

    match fn {
        type_ => || ffi::reckless_cell_renderer_get_type(),
    }
}

impl RecklessCellRenderer {
    pub fn new() -> RecklessCellRenderer {
        //assert_initialized_main_thread!(); // FIXME private module
        if !::gtk::is_initialized_main_thread() {
            if ::gtk::is_initialized() {
                panic!("GTK may only be used from the main thread.");
            } else {
                panic!("GTK has not been initialized. Call `gtk::init` first.");
            }
        }
        unsafe { Object::from_glib_none(ffi::reckless_cell_renderer_new()).unsafe_cast() }
    }
}
impl Default for RecklessCellRenderer {
    fn default() -> Self {
        Self::new()
    }
}
pub trait RecklessCellRendererExt {}

impl<O: IsA<RecklessCellRenderer>> RecklessCellRendererExt for O {}
