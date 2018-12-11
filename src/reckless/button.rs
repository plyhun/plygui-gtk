use glib::object::{Downcast, IsA};
use glib::translate::*;
use gtk::{Buildable, Container, Button, Widget};

use glib_sys as glib_ffi;
use gobject_sys as gobject_ffi;
use gtk_sys as gtk_ffi;

use std::{mem, ptr};

pub mod ffi {
    extern "C" {
        pub fn reckless_button_get_type() -> ::glib_sys::GType;
        pub fn reckless_button_new() -> *mut ::gtk_sys::GtkWidget;
    }

    #[repr(C)]
    pub struct GtkRecklessButtonClass {
        _truncated_record_marker: ::libc::c_void,
        // /*Ignored*/field parent_class has incomplete type
    }
    impl ::std::fmt::Debug for GtkRecklessButtonClass {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.debug_struct(&format!("GtkRecklessButtonClass @ {:?}", self as *const _)).finish()
        }
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct GtkRecklessButton {
        pub container: ::gtk_sys::GtkButton,
    }
    impl ::std::fmt::Debug for GtkRecklessButton {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.debug_struct(&format!("GtkRecklessButton @ {:?}", self as *const _)).field("container", &self.container).finish()
        }
    }
}

glib_wrapper! {
    pub struct RecklessButton(Object<ffi::GtkRecklessButton, ffi::GtkRecklessButtonClass>): [
         Button => gtk_ffi::GtkButton,
         Container => gtk_ffi::GtkContainer,
         Widget => gtk_ffi::GtkWidget,
         Buildable => gtk_ffi::GtkBuildable,
    ];

    match fn {
        get_type => || ffi::reckless_button_get_type(),
    }
}

impl RecklessButton {
    pub fn new() -> RecklessButton {
        //assert_initialized_main_thread!(); // private module
        if !::gtk::is_initialized_main_thread() {
            if ::gtk::is_initialized() {
                panic!("GTK may only be used from the main thread.");
            } else {
                panic!("GTK has not been initialized. Call `gtk::init` first.");
            }
        }
        unsafe { Widget::from_glib_none(ffi::reckless_button_new()).downcast_unchecked() }
    }
}
impl Default for RecklessButton {
    fn default() -> Self {
        Self::new()
    }
}
pub trait RecklessButtonExt {}

impl<O: IsA<RecklessButton>> RecklessButtonExt for O {}
