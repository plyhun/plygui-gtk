use glib::object::{Downcast, IsA};
use glib::translate::*;
use gtk::{Buildable, Container, Frame, Widget};

use glib_sys as glib_ffi;
use gobject_sys as gobject_ffi;
use gtk_sys as gtk_ffi;

use std::{mem, ptr};

pub mod ffi {
    extern "C" {
        pub fn reckless_frame_get_type() -> ::glib_sys::GType;
        pub fn reckless_frame_new() -> *mut ::gtk_sys::GtkWidget;
    }

    #[repr(C)]
    pub struct GtkRecklessFrameClass {
        _truncated_record_marker: ::libc::c_void,
        // /*Ignored*/field parent_class has incomplete type
    }
    impl ::std::fmt::Debug for GtkRecklessFrameClass {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.debug_struct(&format!("GtkRecklessFrameClass @ {:?}", self as *const _)).finish()
        }
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct GtkRecklessFrame {
        pub container: ::gtk_sys::GtkFrame,
    }
    impl ::std::fmt::Debug for GtkRecklessFrame {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.debug_struct(&format!("GtkRecklessFrame @ {:?}", self as *const _)).field("container", &self.container).finish()
        }
    }
}

glib_wrapper! {
    pub struct RecklessFrame(Object<ffi::GtkRecklessFrame, ffi::GtkRecklessFrameClass>): [
         Frame => gtk_ffi::GtkFrame,
         Container => gtk_ffi::GtkContainer,
         Widget => gtk_ffi::GtkWidget,
         Buildable => gtk_ffi::GtkBuildable,
    ];

    match fn {
        get_type => || ffi::reckless_frame_get_type(),
    }
}

impl RecklessFrame {
    pub fn new() -> RecklessFrame {
        //assert_initialized_main_thread!(); // private module
        if !::gtk::is_initialized_main_thread() {
            if ::gtk::is_initialized() {
                panic!("GTK may only be used from the main thread.");
            } else {
                panic!("GTK has not been initialized. Call `gtk::init` first.");
            }
        }
        unsafe { Widget::from_glib_none(ffi::reckless_frame_new()).downcast_unchecked() }
    }
}
impl Default for RecklessFrame {
    fn default() -> Self {
        Self::new()
    }
}
pub trait RecklessFrameExt {}

impl<O: IsA<RecklessFrame>> RecklessFrameExt for O {}
