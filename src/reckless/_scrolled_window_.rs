
            use glib::object::{Downcast, IsA};
            use glib::translate::*;
            use gtk::{ ScrolledWindow, Buildable, Container, Widget };
            
            use glib_sys as glib_ffi;
            use gobject_sys as gobject_ffi;
            use gtk_sys as gtk_ffi;
            
            use std::{mem, ptr};
            
            pub mod ffi {
                extern "C" {
                    pub fn reckless_scrolled_window_get_type() -> ::glib_sys::GType;
                    pub fn reckless_scrolled_window_new() -> *mut ::gtk_sys::GtkWidget;
                }
            
                #[repr(C)]
                pub struct GtkRecklessScrolledWindowClass {
                    _truncated_record_marker: ::libc::c_void,
                    // /*Ignored*/field parent_class has incomplete type
                }
                impl ::std::fmt::Debug for GtkRecklessScrolledWindowClass {
                    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        f.debug_struct(&format!("GtkRecklessScrolledWindowClass @ {:?}", self as *const _)).finish()
                    }
                }
            
                #[repr(C)]
                #[derive(Copy, Clone)]
                pub struct GtkRecklessScrolledWindow {
                    pub container: ::gtk_sys::GtkScrolledWindow,
                }
                impl ::std::fmt::Debug for GtkRecklessScrolledWindow {
                    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        f.debug_struct(&format!("GtkRecklessScrolledWindow @ {:?}", self as *const _)).field("container", &self.container).finish()
                    }
                }
            }
            
            glib_wrapper! {
                pub struct RecklessScrolledWindow(Object<ffi::GtkRecklessScrolledWindow, ffi::GtkRecklessScrolledWindowClass>): [
                     ScrolledWindow => gtk_ffi::GtkScrolledWindow,
                     Container => gtk_ffi::GtkContainer,
                     Widget => gtk_ffi::GtkWidget,
                     Buildable => gtk_ffi::GtkBuildable,
                ];
            
                match fn {
                    get_type => || ffi::reckless_scrolled_window_get_type(),
                }
            }
            
            impl RecklessScrolledWindow {
                pub fn new() -> RecklessScrolledWindow {
                    if !::gtk::is_initialized_main_thread() {
                        if ::gtk::is_initialized() {
                            panic!("GTK may only be used from the main thread.");
                        } else {
                            panic!("GTK has not been initialized. Call `gtk::init` first.");
                        }
                    }
                    unsafe { Widget::from_glib_none(ffi::reckless_scrolled_window_new()).downcast_unchecked() }
                }
            }
            impl Default for RecklessScrolledWindow {
                fn default() -> Self {
                    Self::new()
                }
            }
            pub trait RecklessScrolledWindowExt {}
            
            impl<O: IsA<RecklessScrolledWindow>> RecklessScrolledWindowExt for O {}    
        