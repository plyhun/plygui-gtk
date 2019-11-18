
            use glib::object::{Downcast, IsA};
            use glib::translate::*;
            use gtk::{ TreeView, Buildable, Container, Widget };
            
            use glib_sys as glib_ffi;
            use gobject_sys as gobject_ffi;
            use gtk_sys as gtk_ffi;
            
            use std::{mem, ptr};
            
            pub mod ffi {
                extern "C" {
                    pub fn reckless_tree_view_get_type() -> ::glib_sys::GType;
                    pub fn reckless_tree_view_new() -> *mut ::gtk_sys::GtkWidget;
                }
            
                #[repr(C)]
                pub struct GtkRecklessTreeViewClass {
                    _truncated_record_marker: ::libc::c_void,
                    // /*Ignored*/field parent_class has incomplete type
                }
                impl ::std::fmt::Debug for GtkRecklessTreeViewClass {
                    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        f.debug_struct(&format!("GtkRecklessTreeViewClass @ {:?}", self as *const _)).finish()
                    }
                }
            
                #[repr(C)]
                #[derive(Copy, Clone)]
                pub struct GtkRecklessTreeView {
                    pub container: ::gtk_sys::GtkTreeView,
                }
                impl ::std::fmt::Debug for GtkRecklessTreeView {
                    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        f.debug_struct(&format!("GtkRecklessTreeView @ {:?}", self as *const _)).field("container", &self.container).finish()
                    }
                }
            }
            
            glib_wrapper! {
                pub struct RecklessTreeView(Object<ffi::GtkRecklessTreeView, ffi::GtkRecklessTreeViewClass>): [
                     TreeView => gtk_ffi::GtkTreeView,
                     Container => gtk_ffi::GtkContainer,
                     Widget => gtk_ffi::GtkWidget,
                     Buildable => gtk_ffi::GtkBuildable,
                ];
            
                match fn {
                    get_type => || ffi::reckless_tree_view_get_type(),
                }
            }
            
            impl RecklessTreeView {
                pub fn new() -> RecklessTreeView {
                    if !::gtk::is_initialized_main_thread() {
                        if ::gtk::is_initialized() {
                            panic!("GTK may only be used from the main thread.");
                        } else {
                            panic!("GTK has not been initialized. Call `gtk::init` first.");
                        }
                    }
                    unsafe { Widget::from_glib_none(ffi::reckless_tree_view_new()).downcast_unchecked() }
                }
            }
            impl Default for RecklessTreeView {
                fn default() -> Self {
                    Self::new()
                }
            }
            pub trait RecklessTreeViewExt {}
            
            impl<O: IsA<RecklessTreeView>> RecklessTreeViewExt for O {}    
        