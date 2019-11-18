
            use glib::object::{Downcast, IsA};
            use glib::translate::*;
            use gtk::{ ListBox, Buildable, Container, Widget };
            
            use glib_sys as glib_ffi;
            use gobject_sys as gobject_ffi;
            use gtk_sys as gtk_ffi;
            
            use std::{mem, ptr};
            
            pub mod ffi {
                extern "C" {
                    pub fn reckless_list_box_get_type() -> ::glib_sys::GType;
                    pub fn reckless_list_box_new() -> *mut ::gtk_sys::GtkWidget;
                }
            
                #[repr(C)]
                pub struct GtkRecklessListBoxClass {
                    _truncated_record_marker: ::libc::c_void,
                    // /*Ignored*/field parent_class has incomplete type
                }
                impl ::std::fmt::Debug for GtkRecklessListBoxClass {
                    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        f.debug_struct(&format!("GtkRecklessListBoxClass @ {:?}", self as *const _)).finish()
                    }
                }
            
                #[repr(C)]
                #[derive(Copy, Clone)]
                pub struct GtkRecklessListBox {
                    pub container: ::gtk_sys::GtkListBox,
                }
                impl ::std::fmt::Debug for GtkRecklessListBox {
                    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        f.debug_struct(&format!("GtkRecklessListBox @ {:?}", self as *const _)).field("container", &self.container).finish()
                    }
                }
            }
            
            glib_wrapper! {
                pub struct RecklessListBox(Object<ffi::GtkRecklessListBox, ffi::GtkRecklessListBoxClass>): [
                     ListBox => gtk_ffi::GtkListBox,
                     Container => gtk_ffi::GtkContainer,
                     Widget => gtk_ffi::GtkWidget,
                     Buildable => gtk_ffi::GtkBuildable,
                ];
            
                match fn {
                    get_type => || ffi::reckless_list_box_get_type(),
                }
            }
            
            impl RecklessListBox {
                pub fn new() -> RecklessListBox {
                    if !::gtk::is_initialized_main_thread() {
                        if ::gtk::is_initialized() {
                            panic!("GTK may only be used from the main thread.");
                        } else {
                            panic!("GTK has not been initialized. Call `gtk::init` first.");
                        }
                    }
                    unsafe { Widget::from_glib_none(ffi::reckless_list_box_new()).downcast_unchecked() }
                }
            }
            impl Default for RecklessListBox {
                fn default() -> Self {
                    Self::new()
                }
            }
            pub trait RecklessListBoxExt {}
            
            impl<O: IsA<RecklessListBox>> RecklessListBoxExt for O {}    
        