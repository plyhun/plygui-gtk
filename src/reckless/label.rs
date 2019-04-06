use glib::object::{Cast, IsA};
use glib::translate::*;
use gtk::{Buildable, Container, Label, Widget};

use glib_sys as glib_ffi;
use gobject_sys as gobject_ffi;
use gtk_sys as gtk_ffi;

use std::{mem, ptr};

pub mod ffi {
    extern "C" {
        pub fn reckless_label_get_type() -> ::glib_sys::GType;
        pub fn reckless_label_new() -> *mut ::gtk_sys::GtkWidget;
    }

    #[repr(C)]
    pub struct GtkRecklessLabelClass {
        _truncated_record_marker: ::libc::c_void,
        // /*Ignored*/field parent_class has incomplete type
    }
    impl ::std::fmt::Debug for GtkRecklessLabelClass {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.debug_struct(&format!("GtkRecklessLabelClass @ {:?}", self as *const _)).finish()
        }
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct GtkRecklessLabel {
        pub container: ::gtk_sys::GtkLabel,
    }
    impl ::std::fmt::Debug for GtkRecklessLabel {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.debug_struct(&format!("GtkRecklessLabel @ {:?}", self as *const _)).field("container", &self.container).finish()
        }
    }
}

glib_wrapper! {
    pub struct RecklessLabel(Object<ffi::GtkRecklessLabel, ffi::GtkRecklessLabelClass, RecklessLabelClass>) @extends Label, Container, Widget, @implements Buildable;

    match fn {
        get_type => || ffi::reckless_label_get_type(),
    }
}

impl RecklessLabel {
    pub fn new() -> RecklessLabel {
        //assert_initialized_main_thread!(); // private module
        if !::gtk::is_initialized_main_thread() {
            if ::gtk::is_initialized() {
                panic!("GTK may only be used from the main thread.");
            } else {
                panic!("GTK has not been initialized. Call `gtk::init` first.");
            }
        }
        unsafe { Widget::from_glib_none(ffi::reckless_label_new()).unsafe_cast() }
    }
}
impl Default for RecklessLabel {
    fn default() -> Self {
        Self::new()
    }
}
pub trait RecklessLabelExt {}

impl<O: IsA<RecklessLabel>> RecklessLabelExt for O {}
