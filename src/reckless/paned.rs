use glib::object::{Cast, IsA};
use glib::translate::*;
use gtk::{Buildable, Container, Paned, Widget};

use glib_sys as glib_ffi;
use gobject_sys as gobject_ffi;
use gtk_sys as gtk_ffi;

use std::{mem, ptr};

pub mod ffi {
    extern "C" {
        pub fn reckless_paned_get_type() -> ::glib_sys::GType;
        pub fn reckless_paned_new() -> *mut ::gtk_sys::GtkWidget;
    }

    #[repr(C)]
    pub struct GtkRecklessPanedClass {
        _truncated_record_marker: ::libc::c_void,
        // /*Ignored*/field parent_class has incomplete type
    }
    impl ::std::fmt::Debug for GtkRecklessPanedClass {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.debug_struct(&format!("GtkRecklessPanedClass @ {:?}", self as *const _)).finish()
        }
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct GtkRecklessPaned {
        pub container: ::gtk_sys::GtkPaned,
    }
    impl ::std::fmt::Debug for GtkRecklessPaned {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.debug_struct(&format!("GtkRecklessPaned @ {:?}", self as *const _)).field("container", &self.container).finish()
        }
    }
}

glib_wrapper! {
    pub struct RecklessPaned(Object<ffi::GtkRecklessPaned, ffi::GtkRecklessPanedClass, RecklessPanelClass>) @extends Paned, Container, Widget, @implements Buildable;

    match fn {
        get_type => || ffi::reckless_paned_get_type(),
    }
}

impl RecklessPaned {
    pub fn new() -> RecklessPaned {
        //assert_initialized_main_thread!(); // private module
        if !::gtk::is_initialized_main_thread() {
            if ::gtk::is_initialized() {
                panic!("GTK may only be used from the main thread.");
            } else {
                panic!("GTK has not been initialized. Call `gtk::init` first.");
            }
        }
        unsafe { Widget::from_glib_none(ffi::reckless_paned_new()).unsafe_cast() }
    }
}
impl Default for RecklessPaned {
    fn default() -> Self {
        Self::new()
    }
}
pub trait RecklessPanedExt {}

impl<O: IsA<RecklessPaned>> RecklessPanedExt for O {}
