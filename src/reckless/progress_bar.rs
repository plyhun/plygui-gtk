use glib::object::{Downcast, IsA};
use glib::translate::*;
use gtk::{Buildable, ProgressBar, Container, Widget};

use glib_sys as glib_ffi;
use gobject_sys as gobject_ffi;
use gtk_sys as gtk_ffi;

use std::{mem, ptr};

pub mod ffi {
    extern "C" {
        pub fn reckless_progress_bar_get_type() -> ::glib_sys::GType;
        pub fn reckless_progress_bar_new() -> *mut ::gtk_sys::GtkWidget;
    }

    #[repr(C)]
    pub struct GtkRecklessProgressBarClass {
        _truncated_record_marker: ::libc::c_void,
        // /*Ignored*/field parent_class has incomplete type
    }
    impl ::std::fmt::Debug for GtkRecklessProgressBarClass {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.debug_struct(&format!("GtkRecklessProgressBarClass @ {:?}", self as *const _)).finish()
        }
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct GtkRecklessProgressBar {
        pub container: ::gtk_sys::GtkProgressBar,
    }
    impl ::std::fmt::Debug for GtkRecklessProgressBar {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.debug_struct(&format!("GtkRecklessProgressBar @ {:?}", self as *const _)).field("container", &self.container).finish()
        }
    }
}

glib_wrapper! {
    pub struct RecklessProgressBar(Object<ffi::GtkRecklessProgressBar, ffi::GtkRecklessProgressBarClass>): [
         ProgressBar => gtk_ffi::GtkProgressBar,
         Container => gtk_ffi::GtkContainer,
         Widget => gtk_ffi::GtkWidget,
         Buildable => gtk_ffi::GtkBuildable,
    ];

    match fn {
        get_type => || ffi::reckless_progress_bar_get_type(),
    }
}

impl RecklessProgressBar {
    pub fn new() -> RecklessProgressBar {
        //assert_initialized_main_thread!(); // private module
        if !::gtk::is_initialized_main_thread() {
            if ::gtk::is_initialized() {
                panic!("GTK may only be used from the main thread.");
            } else {
                panic!("GTK has not been initialized. Call `gtk::init` first.");
            }
        }
        unsafe { Widget::from_glib_none(ffi::reckless_progress_bar_new()).downcast_unchecked() }
    }
}
impl Default for RecklessProgressBar {
    fn default() -> Self {
        Self::new()
    }
}
pub trait RecklessProgressBarExt {}

impl<O: IsA<RecklessProgressBar>> RecklessProgressBarExt for O {}
