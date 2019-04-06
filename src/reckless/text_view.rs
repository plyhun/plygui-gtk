use glib::object::{Cast, IsA};
use glib::translate::*;
use gtk::{Buildable, Container, TextView, Widget};

use glib_sys as glib_ffi;
use gobject_sys as gobject_ffi;
use gtk_sys as gtk_ffi;

use std::{mem, ptr};

pub mod ffi {
    extern "C" {
        pub fn reckless_text_view_get_type() -> ::glib_sys::GType;
        pub fn reckless_text_view_new() -> *mut ::gtk_sys::GtkWidget;
    }

    #[repr(C)]
    pub struct GtkRecklessTextViewClass {
        _truncated_record_marker: ::libc::c_void,
        // /*Ignored*/field parent_class has incomplete type
    }
    impl ::std::fmt::Debug for GtkRecklessTextViewClass {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.debug_struct(&format!("GtkRecklessTextViewClass @ {:?}", self as *const _)).finish()
        }
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct GtkRecklessTextView {
        pub container: ::gtk_sys::GtkTextView,
    }
    impl ::std::fmt::Debug for GtkRecklessTextView {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.debug_struct(&format!("GtkRecklessTextView @ {:?}", self as *const _)).field("container", &self.container).finish()
        }
    }
}

glib_wrapper! {
    pub struct RecklessTextView(Object<ffi::GtkRecklessTextView, ffi::GtkRecklessTextViewClass, RecklessTextViewClass>) @extends TextView, Container, Widget, @implements Buildable;

    match fn {
        get_type => || ffi::reckless_text_view_get_type(),
    }
}

impl RecklessTextView {
    pub fn new() -> RecklessTextView {
        //assert_initialized_main_thread!(); // private module
        if !::gtk::is_initialized_main_thread() {
            if ::gtk::is_initialized() {
                panic!("GTK may only be used from the main thread.");
            } else {
                panic!("GTK has not been initialized. Call `gtk::init` first.");
            }
        }
        unsafe { Widget::from_glib_none(ffi::reckless_text_view_new()).unsafe_cast() }
    }
}
impl Default for RecklessTextView {
    fn default() -> Self {
        Self::new()
    }
}
pub trait RecklessTextViewExt {}

impl<O: IsA<RecklessTextView>> RecklessTextViewExt for O {}
