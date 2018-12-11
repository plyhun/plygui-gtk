use glib::object::{Downcast, IsA};
use glib::translate::*;
use gtk::{Buildable, Container, Box, Widget};

use glib_sys as glib_ffi;
use gobject_sys as gobject_ffi;
use gtk_sys as gtk_ffi;

use std::{mem, ptr};

pub mod ffi {
    extern "C" {
        pub fn reckless_box_get_type() -> ::glib_sys::GType;
        pub fn reckless_box_new() -> *mut ::gtk_sys::GtkWidget;
    }

    #[repr(C)]
    pub struct GtkRecklessBoxClass {
        _truncated_record_marker: ::libc::c_void,
        // /*Ignored*/field parent_class has incomplete type
    }
    impl ::std::fmt::Debug for GtkRecklessBoxClass {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.debug_struct(&format!("GtkRecklessBoxClass @ {:?}", self as *const _)).finish()
        }
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct GtkRecklessBox {
        pub container: ::gtk_sys::GtkBox,
    }
    impl ::std::fmt::Debug for GtkRecklessBox {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.debug_struct(&format!("GtkRecklessBox @ {:?}", self as *const _)).field("container", &self.container).finish()
        }
    }
}

glib_wrapper! {
    pub struct RecklessBox(Object<ffi::GtkRecklessBox, ffi::GtkRecklessBoxClass>): [
         Box => gtk_ffi::GtkBox,
         Container => gtk_ffi::GtkContainer,
         Widget => gtk_ffi::GtkWidget,
         Buildable => gtk_ffi::GtkBuildable,
    ];

    match fn {
        get_type => || ffi::reckless_box_get_type(),
    }
}

impl RecklessBox {
    pub fn new() -> RecklessBox {
        //assert_initialized_main_thread!(); // private module
        if !::gtk::is_initialized_main_thread() {
            if ::gtk::is_initialized() {
                panic!("GTK may only be used from the main thread.");
            } else {
                panic!("GTK has not been initialized. Call `gtk::init` first.");
            }
        }
        unsafe { Widget::from_glib_none(ffi::reckless_box_new()).downcast_unchecked() }
    }
}
impl Default for RecklessBox {
    fn default() -> Self {
        Self::new()
    }
}
pub trait RecklessBoxExt {}

impl<O: IsA<RecklessBox>> RecklessBoxExt for O {}
