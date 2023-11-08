// This file was pulled out of the latest gtk-rs version before the deprecation.
// https://github.com/gtk-rs/gtk/commit/e44b7426bf9fa9262495b0aa82506121fab904e4#diff-214fbb38ff7af8682d59184bfea53d81b06af220dd5670ef813bbc83c50272a2

use gtk::{ImageType, Orientation, Tooltip, Menu};
use gdk;
use gdk_pixbuf;
use gdk_sys;
use gio;
use glib;
use glib::GString;
use glib::StaticType;
use glib::Value;
use glib::object::Cast;
use glib::object::IsA;
use glib::object::ObjectExt;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_sys;
use gobject_sys;
use gtk_sys as ffi;
use libc;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

wrapper! {
    pub struct StatusIcon(Object<ffi::GtkStatusIcon, ffi::GtkStatusIconClass>);

    match fn {
        type_ => || ffi::gtk_status_icon_get_type(),
    }
}

pub const NONE_STATUS_ICON: Option<&StatusIcon> = None;

impl StatusIcon {
    pub fn from_file(filename: &str) -> StatusIcon {
        if !::gtk::is_initialized_main_thread() {
            if ::gtk::is_initialized() {
                panic!("GTK may only be used from the main thread.");
            } else {
                panic!("GTK has not been initialized. Call `gtk::init` first.");
            }
        }
        unsafe { StatusIcon::from_glib_none(ffi::gtk_status_icon_new_from_file(filename.to_glib_none().0)).unsafe_cast() }
    }
    pub fn from_gicon(gicon: &gio::Icon) -> StatusIcon {
        if !::gtk::is_initialized_main_thread() {
            if ::gtk::is_initialized() {
                panic!("GTK may only be used from the main thread.");
            } else {
                panic!("GTK has not been initialized. Call `gtk::init` first.");
            }
        }
        unsafe { StatusIcon::from_glib_none(ffi::gtk_status_icon_new_from_gicon(gicon.to_glib_none().0)).unsafe_cast() }
    }
    pub fn from_icon_name(icon_name: &str) -> StatusIcon {
        if !::gtk::is_initialized_main_thread() {
            if ::gtk::is_initialized() {
                panic!("GTK may only be used from the main thread.");
            } else {
                panic!("GTK has not been initialized. Call `gtk::init` first.");
            }
        }
        unsafe { StatusIcon::from_glib_none(ffi::gtk_status_icon_new_from_icon_name(icon_name.to_glib_none().0)).unsafe_cast() }
    }
    pub fn from_pixbuf(pixbuf: &gdk_pixbuf::Pixbuf) -> StatusIcon {
        if !::gtk::is_initialized_main_thread() {
            if ::gtk::is_initialized() {
                panic!("GTK may only be used from the main thread.");
            } else {
                panic!("GTK has not been initialized. Call `gtk::init` first.");
            }
        }
        unsafe { StatusIcon::from_glib_none(ffi::gtk_status_icon_new_from_pixbuf(pixbuf.to_glib_none().0)).unsafe_cast() }
    }
    pub fn from_stock(stock_id: &str) -> StatusIcon {
        if !::gtk::is_initialized_main_thread() {
            if ::gtk::is_initialized() {
                panic!("GTK may only be used from the main thread.");
            } else {
                panic!("GTK has not been initialized. Call `gtk::init` first.");
            }
        }
        unsafe { StatusIcon::from_glib_none(ffi::gtk_status_icon_new_from_stock(stock_id.to_glib_none().0)).unsafe_cast() }
    }
}

pub trait StatusIconExt: 'static {
    fn position_menu(&self, x: &mut i32, y: &mut i32, menu: &Menu) -> bool;

    fn is_embedded(&self) -> bool;

    fn set_file(&self, file: Option<&str>);

    fn gicon(&self) -> Option<gio::Icon>;

    fn set_gicon(&self, gicon: Option<&gio::Icon>);

    fn has_tooltip(&self) -> bool;

    fn set_has_tooltip(&self, has_tooltip: bool);

    fn icon_name(&self) -> Option<GString>;

    fn set_icon_name(&self, icon_name: Option<&str>);

    fn orientation(&self) -> Orientation;

    fn pixbuf(&self) -> Option<gdk_pixbuf::Pixbuf>;

    fn set_pixbuf(&self, pixbuf: Option<&gdk_pixbuf::Pixbuf>);

    fn screen(&self) -> Option<gdk::Screen>;

    fn set_screen(&self, screen: Option<&gdk::Screen>);

    fn size(&self) -> i32;

    fn storage_type(&self) -> ImageType;

    fn title(&self) -> Option<GString>;

    fn set_title(&self, title: Option<&str>);

    fn tooltip_markup(&self) -> Option<GString>;

    fn set_tooltip_markup(&self, tooltip_markup: Option<&str>);

    fn tooltip_text(&self) -> Option<GString>;

    fn set_tooltip_text(&self, tooltip_text: Option<&str>);

    fn visible(&self) -> bool;

    fn set_visible(&self, visible: bool);

    fn connect_activate<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_activate(&self);

    fn connect_button_press_event<F: Fn(&Self, &gdk::EventButton) -> bool + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_button_release_event<F: Fn(&Self, &gdk::EventButton) -> bool + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_popup_menu<F: Fn(&Self, u32, u32) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_popup_menu(&self, button: u32, activate_time: u32);

    fn connect_query_tooltip<F: Fn(&Self, i32, i32, bool, &Tooltip) -> bool + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_scroll_event<F: Fn(&Self, &gdk::EventScroll) -> bool + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_size_changed<F: Fn(&Self, i32) -> bool + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_embedded_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_file_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_gicon_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_has_tooltip_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_icon_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_orientation_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_pixbuf_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_screen_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_storage_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_title_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_tooltip_markup_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_tooltip_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_visible_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<StatusIcon>> StatusIconExt for O {
    fn position_menu(&self, x: &mut i32, y: &mut i32, menu: &Menu) -> bool {
        let mut push_in = 0;
        unsafe {
            ffi::gtk_status_icon_position_menu(menu.to_glib_none().0, x, y, &mut push_in, self.as_ref().to_glib_none().0);
        }
        if push_in == 0 { false } else { true }
    }

    fn is_embedded(&self) -> bool {
        unsafe { from_glib(ffi::gtk_status_icon_is_embedded(self.as_ref().to_glib_none().0)) }
    }

    fn set_file(&self, file: Option<&str>) {
        unsafe {
            ffi::gtk_status_icon_set_from_file(self.as_ref().to_glib_none().0, file.to_glib_none().0);
        }
    }

    fn gicon(&self) -> Option<gio::Icon> {
        unsafe { from_glib_none(ffi::gtk_status_icon_get_gicon(self.as_ref().to_glib_none().0)) }
    }

    fn set_gicon(&self, gicon: Option<&gio::Icon>) {
        unsafe {
            ffi::gtk_status_icon_set_from_gicon(self.as_ref().to_glib_none().0, gicon.to_glib_none().0);
        }
    }

    fn has_tooltip(&self) -> bool {
        unsafe { from_glib(ffi::gtk_status_icon_get_has_tooltip(self.as_ref().to_glib_none().0)) }
    }

    fn set_has_tooltip(&self, has_tooltip: bool) {
        unsafe {
            ffi::gtk_status_icon_set_has_tooltip(self.as_ref().to_glib_none().0, has_tooltip.into_glib());
        }
    }

    fn icon_name(&self) -> Option<GString> {
        unsafe { from_glib_none(ffi::gtk_status_icon_get_icon_name(self.as_ref().to_glib_none().0)) }
    }

    fn set_icon_name(&self, icon_name: Option<&str>) {
        unsafe {
            ffi::gtk_status_icon_set_from_icon_name(self.as_ref().to_glib_none().0, icon_name.to_glib_none().0);
        }
    }

    fn orientation(&self) -> Orientation {
        unsafe {
            let mut value = Value::from_type(<Orientation as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"orientation\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn pixbuf(&self) -> Option<gdk_pixbuf::Pixbuf> {
        unsafe { from_glib_none(ffi::gtk_status_icon_get_pixbuf(self.as_ref().to_glib_none().0)) }
    }

    fn set_pixbuf(&self, pixbuf: Option<&gdk_pixbuf::Pixbuf>) {
        unsafe {
            ffi::gtk_status_icon_set_from_pixbuf(self.as_ref().to_glib_none().0, pixbuf.to_glib_none().0);
        }
    }

    fn screen(&self) -> Option<gdk::Screen> {
        unsafe { from_glib_none(ffi::gtk_status_icon_get_screen(self.as_ref().to_glib_none().0)) }
    }

    fn set_screen(&self, screen: Option<&gdk::Screen>) {
        unsafe {
            ffi::gtk_status_icon_set_screen(self.as_ref().to_glib_none().0, screen.to_glib_none().0);
        }
    }

    fn size(&self) -> i32 {
        unsafe { ffi::gtk_status_icon_get_size(self.as_ref().to_glib_none().0) }
    }

    fn storage_type(&self) -> ImageType {
        unsafe { from_glib(ffi::gtk_status_icon_get_storage_type(self.as_ref().to_glib_none().0)) }
    }

    fn title(&self) -> Option<GString> {
        unsafe { from_glib_none(ffi::gtk_status_icon_get_title(self.as_ref().to_glib_none().0)) }
    }

    fn set_title(&self, title: Option<&str>) {
        unsafe {
            ffi::gtk_status_icon_set_title(self.as_ref().to_glib_none().0, title.to_glib_none().0);
        }
    }

    fn tooltip_markup(&self) -> Option<GString> {
        unsafe { from_glib_none(ffi::gtk_status_icon_get_tooltip_markup(self.as_ref().to_glib_none().0)) }
    }

    fn set_tooltip_markup(&self, tooltip_markup: Option<&str>) {
        unsafe {
            ffi::gtk_status_icon_set_tooltip_markup(self.as_ref().to_glib_none().0, tooltip_markup.to_glib_none().0);
        }
    }

    fn tooltip_text(&self) -> Option<GString> {
        unsafe { from_glib_none(ffi::gtk_status_icon_get_tooltip_text(self.as_ref().to_glib_none().0)) }
    }

    fn set_tooltip_text(&self, tooltip_text: Option<&str>) {
        unsafe {
            ffi::gtk_status_icon_set_tooltip_text(self.as_ref().to_glib_none().0, tooltip_text.to_glib_none().0);
        }
    }

    fn visible(&self) -> bool {
        unsafe { from_glib(ffi::gtk_status_icon_get_visible(self.as_ref().to_glib_none().0)) }
    }

    fn set_visible(&self, visible: bool) {
        unsafe {
            ffi::gtk_status_icon_set_visible(self.as_ref().to_glib_none().0, visible.into_glib());
        }
    }

    fn connect_activate<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"activate\0".as_ptr() as *const _,
                Some(transmute(activate_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn emit_activate(&self) {
        self.emit_by_name::<()>("activate", &[]);
    }

    fn connect_button_press_event<F: Fn(&Self, &gdk::EventButton) -> bool + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"button-press-event\0".as_ptr() as *const _,
                Some(transmute(button_press_event_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_button_release_event<F: Fn(&Self, &gdk::EventButton) -> bool + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"button-release-event\0".as_ptr() as *const _,
                Some(transmute(button_release_event_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_popup_menu<F: Fn(&Self, u32, u32) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"popup-menu\0".as_ptr() as *const _,
                Some(transmute(popup_menu_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn emit_popup_menu(&self, button: u32, activate_time: u32) {
        self.emit_by_name::<()>("popup-menu", &[&button, &activate_time]);
    }

    fn connect_query_tooltip<F: Fn(&Self, i32, i32, bool, &Tooltip) -> bool + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"query-tooltip\0".as_ptr() as *const _,
                Some(transmute(query_tooltip_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_scroll_event<F: Fn(&Self, &gdk::EventScroll) -> bool + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"scroll-event\0".as_ptr() as *const _,
                Some(transmute(scroll_event_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_size_changed<F: Fn(&Self, i32) -> bool + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"size-changed\0".as_ptr() as *const _,
                Some(transmute(size_changed_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_embedded_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::embedded\0".as_ptr() as *const _,
                Some(transmute(notify_embedded_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_file_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::file\0".as_ptr() as *const _,
                Some(transmute(notify_file_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_gicon_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::gicon\0".as_ptr() as *const _,
                Some(transmute(notify_gicon_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_has_tooltip_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::has-tooltip\0".as_ptr() as *const _,
                Some(transmute(notify_has_tooltip_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_icon_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::icon-name\0".as_ptr() as *const _,
                Some(transmute(notify_icon_name_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_orientation_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::orientation\0".as_ptr() as *const _,
                Some(transmute(notify_orientation_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_pixbuf_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::pixbuf\0".as_ptr() as *const _,
                Some(transmute(notify_pixbuf_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_screen_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::screen\0".as_ptr() as *const _,
                Some(transmute(notify_screen_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::size\0".as_ptr() as *const _,
                Some(transmute(notify_size_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_storage_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::storage-type\0".as_ptr() as *const _,
                Some(transmute(notify_storage_type_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_title_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::title\0".as_ptr() as *const _,
                Some(transmute(notify_title_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_tooltip_markup_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::tooltip-markup\0".as_ptr() as *const _,
                Some(transmute(notify_tooltip_markup_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_tooltip_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::tooltip-text\0".as_ptr() as *const _,
                Some(transmute(notify_tooltip_text_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_visible_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::visible\0".as_ptr() as *const _,
                Some(transmute(notify_visible_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe extern "C" fn activate_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkStatusIcon, f: glib_sys::gpointer)
where P: IsA<StatusIcon> {
    let f: &F = &*(f as *const F);
    f(&StatusIcon::from_glib_borrow(this).unsafe_cast_ref())
}

unsafe extern "C" fn button_press_event_trampoline<P, F: Fn(&P, &gdk::EventButton) -> bool + 'static>(this: *mut gtk_sys::GtkStatusIcon, event: *mut gdk_sys::GdkEventButton, f: glib_sys::gpointer) -> glib_sys::gboolean
where P: IsA<StatusIcon> {
    let f: &F = &*(f as *const F);
    f(&StatusIcon::from_glib_borrow(this).unsafe_cast_ref(), &from_glib_borrow(event)).into_glib()
}

unsafe extern "C" fn button_release_event_trampoline<P, F: Fn(&P, &gdk::EventButton) -> bool + 'static>(this: *mut gtk_sys::GtkStatusIcon, event: *mut gdk_sys::GdkEventButton, f: glib_sys::gpointer) -> glib_sys::gboolean
where P: IsA<StatusIcon> {// This file was generated by gir (https://github.com/gtk-rs/gir)
    // from gir-files (https://github.com/gtk-rs/gir-files)
    // DO NOT EDIT
    
    wrapper! {
        pub struct StatusIcon(Object<gtk_sys::GtkStatusIcon, gtk_sys::GtkStatusIconClass>);
    
        match fn {
            type_ => || gtk_sys::gtk_status_icon_get_type(),
        }
    }
    
    pub const NONE_STATUS_ICON: Option<&StatusIcon> = None;
    
    pub trait StatusIconExt: 'static {
        fn position_menu(&self, x: &mut i32, y: &mut i32, menu: &Menu) -> bool ;

        fn is_embedded(&self) -> bool;
    
        fn set_file(&self, file: Option<&str>);
    
        fn gicon(&self) -> Option<gio::Icon>;
    
        fn set_gicon(&self, gicon: Option<&gio::Icon>);
    
        fn has_tooltip(&self) -> bool;
    
        fn set_has_tooltip(&self, has_tooltip: bool);
    
        fn icon_name(&self) -> Option<GString>;
    
        fn set_icon_name(&self, icon_name: Option<&str>);
    
        fn orientation(&self) -> Orientation;
    
        fn pixbuf(&self) -> Option<gdk_pixbuf::Pixbuf>;
    
        fn set_pixbuf(&self, pixbuf: Option<&gdk_pixbuf::Pixbuf>);
    
        fn screen(&self) -> Option<gdk::Screen>;
    
        fn set_screen(&self, screen: Option<&gdk::Screen>);
    
        fn size(&self) -> i32;
    
        fn storage_type(&self) -> ImageType;
    
        fn title(&self) -> Option<GString>;
    
        fn set_title(&self, title: Option<&str>);
    
        fn tooltip_markup(&self) -> Option<GString>;
    
        fn set_tooltip_markup(&self, tooltip_markup: Option<&str>);
    
        fn tooltip_text(&self) -> Option<GString>;
    
        fn set_tooltip_text(&self, tooltip_text: Option<&str>);
    
        fn visible(&self) -> bool;
    
        fn set_visible(&self, visible: bool);
    
        fn connect_activate<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
    
        fn emit_activate(&self);
    
        fn connect_button_press_event<F: Fn(&Self, &gdk::EventButton) -> bool + 'static>(&self, f: F) -> SignalHandlerId;
    
        fn connect_button_release_event<F: Fn(&Self, &gdk::EventButton) -> bool + 'static>(&self, f: F) -> SignalHandlerId;
    
        fn connect_popup_menu<F: Fn(&Self, u32, u32) + 'static>(&self, f: F) -> SignalHandlerId;
    
        fn emit_popup_menu(&self, button: u32, activate_time: u32);
    
        fn connect_query_tooltip<F: Fn(&Self, i32, i32, bool, &Tooltip) -> bool + 'static>(&self, f: F) -> SignalHandlerId;
    
        fn connect_scroll_event<F: Fn(&Self, &gdk::EventScroll) -> bool + 'static>(&self, f: F) -> SignalHandlerId;
    
        fn connect_size_changed<F: Fn(&Self, i32) -> bool + 'static>(&self, f: F) -> SignalHandlerId;
    
        fn connect_embedded_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
    
        fn connect_file_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
    
        fn connect_gicon_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
    
        fn connect_has_tooltip_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
    
        fn connect_icon_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
    
        fn connect_orientation_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
    
        fn connect_pixbuf_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
    
        fn connect_screen_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
    
        fn connect_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
    
        fn connect_storage_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
    
        fn connect_title_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
    
        fn connect_tooltip_markup_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
    
        fn connect_tooltip_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
    
        fn connect_visible_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
    }
    
    impl<O: IsA<StatusIcon>> StatusIconExt for O {
        fn position_menu(&self, x: &mut i32, y: &mut i32, menu: &Menu) -> bool {
            let mut push_in = 0;
            unsafe {
                ffi::gtk_status_icon_position_menu(menu.to_glib_none().0, x, y, &mut push_in, self.as_ref().to_glib_none().0);
            }
            if push_in == 0 { false } else { true }
        }
    
        fn is_embedded(&self) -> bool {
            unsafe { from_glib(ffi::gtk_status_icon_is_embedded(self.as_ref().to_glib_none().0)) }
        }
    
        fn set_file(&self, file: Option<&str>) {
            unsafe {
                ffi::gtk_status_icon_set_from_file(self.as_ref().to_glib_none().0, file.to_glib_none().0);
            }
        }
    
        fn gicon(&self) -> Option<gio::Icon> {
            unsafe { from_glib_none(ffi::gtk_status_icon_get_gicon(self.as_ref().to_glib_none().0)) }
        }
    
        fn set_gicon(&self, gicon: Option<&gio::Icon>) {
            unsafe {
                ffi::gtk_status_icon_set_from_gicon(self.as_ref().to_glib_none().0, gicon.to_glib_none().0);
            }
        }
    
        fn has_tooltip(&self) -> bool {
            unsafe { from_glib(ffi::gtk_status_icon_get_has_tooltip(self.as_ref().to_glib_none().0)) }
        }
    
        fn set_has_tooltip(&self, has_tooltip: bool) {
            unsafe {
                ffi::gtk_status_icon_set_has_tooltip(self.as_ref().to_glib_none().0, has_tooltip.into_glib());
            }
        }
    
        fn icon_name(&self) -> Option<GString> {
            unsafe { from_glib_none(ffi::gtk_status_icon_get_icon_name(self.as_ref().to_glib_none().0)) }
        }
    
        fn set_icon_name(&self, icon_name: Option<&str>) {
            unsafe {
                ffi::gtk_status_icon_set_from_icon_name(self.as_ref().to_glib_none().0, icon_name.to_glib_none().0);
            }
        }
    
        fn orientation(&self) -> Orientation {
            unsafe {
                let mut value = Value::from_type(<Orientation as StaticType>::static_type());
                gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"orientation\0".as_ptr() as *const _, value.to_glib_none_mut().0);
                value.get().unwrap()
            }
        }
    
        fn pixbuf(&self) -> Option<gdk_pixbuf::Pixbuf> {
            unsafe { from_glib_none(ffi::gtk_status_icon_get_pixbuf(self.as_ref().to_glib_none().0)) }
        }
    
        fn set_pixbuf(&self, pixbuf: Option<&gdk_pixbuf::Pixbuf>) {
            unsafe {
                ffi::gtk_status_icon_set_from_pixbuf(self.as_ref().to_glib_none().0, pixbuf.to_glib_none().0);
            }
        }
    
        fn screen(&self) -> Option<gdk::Screen> {
            unsafe { from_glib_none(ffi::gtk_status_icon_get_screen(self.as_ref().to_glib_none().0)) }
        }
    
        fn set_screen(&self, screen: Option<&gdk::Screen>) {
            unsafe {
                ffi::gtk_status_icon_set_screen(self.as_ref().to_glib_none().0, screen.to_glib_none().0);
            }
        }
    
        fn size(&self) -> i32 {
            unsafe { ffi::gtk_status_icon_get_size(self.as_ref().to_glib_none().0) }
        }
    
        fn storage_type(&self) -> ImageType {
            unsafe { from_glib(ffi::gtk_status_icon_get_storage_type(self.as_ref().to_glib_none().0)) }
        }
    
        fn title(&self) -> Option<GString> {
            unsafe { from_glib_none(ffi::gtk_status_icon_get_title(self.as_ref().to_glib_none().0)) }
        }
    
        fn set_title(&self, title: Option<&str>) {
            unsafe {
                ffi::gtk_status_icon_set_title(self.as_ref().to_glib_none().0, title.to_glib_none().0);
            }
        }
    
        fn tooltip_markup(&self) -> Option<GString> {
            unsafe { from_glib_none(ffi::gtk_status_icon_get_tooltip_markup(self.as_ref().to_glib_none().0)) }
        }
    
        fn set_tooltip_markup(&self, tooltip_markup: Option<&str>) {
            unsafe {
                ffi::gtk_status_icon_set_tooltip_markup(self.as_ref().to_glib_none().0, tooltip_markup.to_glib_none().0);
            }
        }
    
        fn tooltip_text(&self) -> Option<GString> {
            unsafe { from_glib_none(ffi::gtk_status_icon_get_tooltip_text(self.as_ref().to_glib_none().0)) }
        }
    
        fn set_tooltip_text(&self, tooltip_text: Option<&str>) {
            unsafe {
                ffi::gtk_status_icon_set_tooltip_text(self.as_ref().to_glib_none().0, tooltip_text.to_glib_none().0);
            }
        }
    
        fn visible(&self) -> bool {
            unsafe { from_glib(ffi::gtk_status_icon_get_visible(self.as_ref().to_glib_none().0)) }
        }
    
        fn set_visible(&self, visible: bool) {
            unsafe {
                ffi::gtk_status_icon_set_visible(self.as_ref().to_glib_none().0, visible.into_glib());
            }
        }
    
        fn connect_activate<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
            unsafe {
                let f: Box_<F> = Box_::new(f);
                connect_raw(self.as_ptr() as *mut _, b"activate\0".as_ptr() as *const _,
                    Some(transmute(activate_trampoline::<Self, F> as usize)), Box_::into_raw(f))
            }
        }
    
        fn emit_activate(&self) {
            self.emit_by_name::<()>("activate", &[]);
        }
    
        fn connect_button_press_event<F: Fn(&Self, &gdk::EventButton) -> bool + 'static>(&self, f: F) -> SignalHandlerId {
            unsafe {
                let f: Box_<F> = Box_::new(f);
                connect_raw(self.as_ptr() as *mut _, b"button-press-event\0".as_ptr() as *const _,
                    Some(transmute(button_press_event_trampoline::<Self, F> as usize)), Box_::into_raw(f))
            }
        }
    
        fn connect_button_release_event<F: Fn(&Self, &gdk::EventButton) -> bool + 'static>(&self, f: F) -> SignalHandlerId {
            unsafe {
                let f: Box_<F> = Box_::new(f);
                connect_raw(self.as_ptr() as *mut _, b"button-release-event\0".as_ptr() as *const _,
                    Some(transmute(button_release_event_trampoline::<Self, F> as usize)), Box_::into_raw(f))
            }
        }
    
        fn connect_popup_menu<F: Fn(&Self, u32, u32) + 'static>(&self, f: F) -> SignalHandlerId {
            unsafe {
                let f: Box_<F> = Box_::new(f);
                connect_raw(self.as_ptr() as *mut _, b"popup-menu\0".as_ptr() as *const _,
                    Some(transmute(popup_menu_trampoline::<Self, F> as usize)), Box_::into_raw(f))
            }
        }
    
        fn emit_popup_menu(&self, button: u32, activate_time: u32) {
            self.emit_by_name::<()>("popup-menu", &[&button, &activate_time]);
        }
    
        fn connect_query_tooltip<F: Fn(&Self, i32, i32, bool, &Tooltip) -> bool + 'static>(&self, f: F) -> SignalHandlerId {
            unsafe {
                let f: Box_<F> = Box_::new(f);
                connect_raw(self.as_ptr() as *mut _, b"query-tooltip\0".as_ptr() as *const _,
                    Some(transmute(query_tooltip_trampoline::<Self, F> as usize)), Box_::into_raw(f))
            }
        }
    
        fn connect_scroll_event<F: Fn(&Self, &gdk::EventScroll) -> bool + 'static>(&self, f: F) -> SignalHandlerId {
            unsafe {
                let f: Box_<F> = Box_::new(f);
                connect_raw(self.as_ptr() as *mut _, b"scroll-event\0".as_ptr() as *const _,
                    Some(transmute(scroll_event_trampoline::<Self, F> as usize)), Box_::into_raw(f))
            }
        }
    
        fn connect_size_changed<F: Fn(&Self, i32) -> bool + 'static>(&self, f: F) -> SignalHandlerId {
            unsafe {
                let f: Box_<F> = Box_::new(f);
                connect_raw(self.as_ptr() as *mut _, b"size-changed\0".as_ptr() as *const _,
                    Some(transmute(size_changed_trampoline::<Self, F> as usize)), Box_::into_raw(f))
            }
        }
    
        fn connect_embedded_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
            unsafe {
                let f: Box_<F> = Box_::new(f);
                connect_raw(self.as_ptr() as *mut _, b"notify::embedded\0".as_ptr() as *const _,
                    Some(transmute(notify_embedded_trampoline::<Self, F> as usize)), Box_::into_raw(f))
            }
        }
    
        fn connect_file_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
            unsafe {
                let f: Box_<F> = Box_::new(f);
                connect_raw(self.as_ptr() as *mut _, b"notify::file\0".as_ptr() as *const _,
                    Some(transmute(notify_file_trampoline::<Self, F> as usize)), Box_::into_raw(f))
            }
        }
    
        fn connect_gicon_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
            unsafe {
                let f: Box_<F> = Box_::new(f);
                connect_raw(self.as_ptr() as *mut _, b"notify::gicon\0".as_ptr() as *const _,
                    Some(transmute(notify_gicon_trampoline::<Self, F> as usize)), Box_::into_raw(f))
            }
        }
    
        fn connect_has_tooltip_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
            unsafe {
                let f: Box_<F> = Box_::new(f);
                connect_raw(self.as_ptr() as *mut _, b"notify::has-tooltip\0".as_ptr() as *const _,
                    Some(transmute(notify_has_tooltip_trampoline::<Self, F> as usize)), Box_::into_raw(f))
            }
        }
    
        fn connect_icon_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
            unsafe {
                let f: Box_<F> = Box_::new(f);
                connect_raw(self.as_ptr() as *mut _, b"notify::icon-name\0".as_ptr() as *const _,
                    Some(transmute(notify_icon_name_trampoline::<Self, F> as usize)), Box_::into_raw(f))
            }
        }
    
        fn connect_orientation_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
            unsafe {
                let f: Box_<F> = Box_::new(f);
                connect_raw(self.as_ptr() as *mut _, b"notify::orientation\0".as_ptr() as *const _,
                    Some(transmute(notify_orientation_trampoline::<Self, F> as usize)), Box_::into_raw(f))
            }
        }
    
        fn connect_pixbuf_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
            unsafe {
                let f: Box_<F> = Box_::new(f);
                connect_raw(self.as_ptr() as *mut _, b"notify::pixbuf\0".as_ptr() as *const _,
                    Some(transmute(notify_pixbuf_trampoline::<Self, F> as usize)), Box_::into_raw(f))
            }
        }
    
        fn connect_screen_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
            unsafe {
                let f: Box_<F> = Box_::new(f);
                connect_raw(self.as_ptr() as *mut _, b"notify::screen\0".as_ptr() as *const _,
                    Some(transmute(notify_screen_trampoline::<Self, F> as usize)), Box_::into_raw(f))
            }
        }
    
        fn connect_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
            unsafe {
                let f: Box_<F> = Box_::new(f);
                connect_raw(self.as_ptr() as *mut _, b"notify::size\0".as_ptr() as *const _,
                    Some(transmute(notify_size_trampoline::<Self, F> as usize)), Box_::into_raw(f))
            }
        }
    
        fn connect_storage_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
            unsafe {
                let f: Box_<F> = Box_::new(f);
                connect_raw(self.as_ptr() as *mut _, b"notify::storage-type\0".as_ptr() as *const _,
                    Some(transmute(notify_storage_type_trampoline::<Self, F> as usize)), Box_::into_raw(f))
            }
        }
    
        fn connect_title_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
            unsafe {
                let f: Box_<F> = Box_::new(f);
                connect_raw(self.as_ptr() as *mut _, b"notify::title\0".as_ptr() as *const _,
                    Some(transmute(notify_title_trampoline::<Self, F> as usize)), Box_::into_raw(f))
            }
        }
    
        fn connect_tooltip_markup_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
            unsafe {
                let f: Box_<F> = Box_::new(f);
                connect_raw(self.as_ptr() as *mut _, b"notify::tooltip-markup\0".as_ptr() as *const _,
                    Some(transmute(notify_tooltip_markup_trampoline::<Self, F> as usize)), Box_::into_raw(f))
            }
        }
    
        fn connect_tooltip_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
            unsafe {
                let f: Box_<F> = Box_::new(f);
                connect_raw(self.as_ptr() as *mut _, b"notify::tooltip-text\0".as_ptr() as *const _,
                    Some(transmute(notify_tooltip_text_trampoline::<Self, F> as usize)), Box_::into_raw(f))
            }
        }
    
        fn connect_visible_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
            unsafe {
                let f: Box_<F> = Box_::new(f);
                connect_raw(self.as_ptr() as *mut _, b"notify::visible\0".as_ptr() as *const _,
                    Some(transmute(notify_visible_trampoline::<Self, F> as usize)), Box_::into_raw(f))
            }
        }
    }
    
    unsafe extern "C" fn activate_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkStatusIcon, f: glib_sys::gpointer)
    where P: IsA<StatusIcon> {
        let f: &F = &*(f as *const F);
        f(&StatusIcon::from_glib_borrow(this).unsafe_cast_ref())
    }
    
    unsafe extern "C" fn button_press_event_trampoline<P, F: Fn(&P, &gdk::EventButton) -> bool + 'static>(this: *mut gtk_sys::GtkStatusIcon, event: *mut gdk_sys::GdkEventButton, f: glib_sys::gpointer) -> glib_sys::gboolean
    where P: IsA<StatusIcon> {
        let f: &F = &*(f as *const F);
        f(&StatusIcon::from_glib_borrow(this).unsafe_cast_ref(), &from_glib_borrow(event)).into_glib()
    }
    
    unsafe extern "C" fn button_release_event_trampoline<P, F: Fn(&P, &gdk::EventButton) -> bool + 'static>(this: *mut gtk_sys::GtkStatusIcon, event: *mut gdk_sys::GdkEventButton, f: glib_sys::gpointer) -> glib_sys::gboolean
    where P: IsA<StatusIcon> {
        let f: &F = &*(f as *const F);
        f(&StatusIcon::from_glib_borrow(this).unsafe_cast_ref(), &from_glib_borrow(event)).into_glib()
    }
    
    unsafe extern "C" fn popup_menu_trampoline<P, F: Fn(&P, u32, u32) + 'static>(this: *mut gtk_sys::GtkStatusIcon, button: libc::c_uint, activate_time: libc::c_uint, f: glib_sys::gpointer)
    where P: IsA<StatusIcon> {
        let f: &F = &*(f as *const F);
        f(&StatusIcon::from_glib_borrow(this).unsafe_cast_ref(), button, activate_time)
    }
    
    unsafe extern "C" fn query_tooltip_trampoline<P, F: Fn(&P, i32, i32, bool, &Tooltip) -> bool + 'static>(this: *mut gtk_sys::GtkStatusIcon, x: libc::c_int, y: libc::c_int, keyboard_mode: glib_sys::gboolean, tooltip: *mut gtk_sys::GtkTooltip, f: glib_sys::gpointer) -> glib_sys::gboolean
    where P: IsA<StatusIcon> {
        let f: &F = &*(f as *const F);
        f(&StatusIcon::from_glib_borrow(this).unsafe_cast_ref(), x, y, from_glib(keyboard_mode), &from_glib_borrow(tooltip)).into_glib()
    }
    
    unsafe extern "C" fn scroll_event_trampoline<P, F: Fn(&P, &gdk::EventScroll) -> bool + 'static>(this: *mut gtk_sys::GtkStatusIcon, event: *mut gdk_sys::GdkEventScroll, f: glib_sys::gpointer) -> glib_sys::gboolean
    where P: IsA<StatusIcon> {
        let f: &F = &*(f as *const F);
        f(&StatusIcon::from_glib_borrow(this).unsafe_cast_ref(), &from_glib_borrow(event)).into_glib()
    }
    
    unsafe extern "C" fn size_changed_trampoline<P, F: Fn(&P, i32) -> bool + 'static>(this: *mut gtk_sys::GtkStatusIcon, size: libc::c_int, f: glib_sys::gpointer) -> glib_sys::gboolean
    where P: IsA<StatusIcon> {
        let f: &F = &*(f as *const F);
        f(&StatusIcon::from_glib_borrow(this).unsafe_cast_ref(), size).into_glib()
    }
    
    unsafe extern "C" fn notify_embedded_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkStatusIcon, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
    where P: IsA<StatusIcon> {
        let f: &F = &*(f as *const F);
        f(&StatusIcon::from_glib_borrow(this).unsafe_cast_ref())
    }
    
    unsafe extern "C" fn notify_file_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkStatusIcon, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
    where P: IsA<StatusIcon> {
        let f: &F = &*(f as *const F);
        f(&StatusIcon::from_glib_borrow(this).unsafe_cast_ref())
    }
    
    unsafe extern "C" fn notify_gicon_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkStatusIcon, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
    where P: IsA<StatusIcon> {
        let f: &F = &*(f as *const F);
        f(&StatusIcon::from_glib_borrow(this).unsafe_cast_ref())
    }
    
    unsafe extern "C" fn notify_has_tooltip_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkStatusIcon, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
    where P: IsA<StatusIcon> {
        let f: &F = &*(f as *const F);
        f(&StatusIcon::from_glib_borrow(this).unsafe_cast_ref())
    }
    
    unsafe extern "C" fn notify_icon_name_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkStatusIcon, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
    where P: IsA<StatusIcon> {
        let f: &F = &*(f as *const F);
        f(&StatusIcon::from_glib_borrow(this).unsafe_cast_ref())
    }
    
    unsafe extern "C" fn notify_orientation_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkStatusIcon, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
    where P: IsA<StatusIcon> {
        let f: &F = &*(f as *const F);
        f(&StatusIcon::from_glib_borrow(this).unsafe_cast_ref())
    }
    
    unsafe extern "C" fn notify_pixbuf_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkStatusIcon, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
    where P: IsA<StatusIcon> {
        let f: &F = &*(f as *const F);
        f(&StatusIcon::from_glib_borrow(this).unsafe_cast_ref())
    }
    
    unsafe extern "C" fn notify_screen_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkStatusIcon, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
    where P: IsA<StatusIcon> {
        let f: &F = &*(f as *const F);
        f(&StatusIcon::from_glib_borrow(this).unsafe_cast_ref())
    }
    
    unsafe extern "C" fn notify_size_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkStatusIcon, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
    where P: IsA<StatusIcon> {
        let f: &F = &*(f as *const F);
        f(&StatusIcon::from_glib_borrow(this).unsafe_cast_ref())
    }
    
    unsafe extern "C" fn notify_storage_type_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkStatusIcon, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
    where P: IsA<StatusIcon> {
        let f: &F = &*(f as *const F);
        f(&StatusIcon::from_glib_borrow(this).unsafe_cast_ref())
    }
    
    unsafe extern "C" fn notify_title_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkStatusIcon, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
    where P: IsA<StatusIcon> {
        let f: &F = &*(f as *const F);
        f(&StatusIcon::from_glib_borrow(this).unsafe_cast_ref())
    }
    
    unsafe extern "C" fn notify_tooltip_markup_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkStatusIcon, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
    where P: IsA<StatusIcon> {
        let f: &F = &*(f as *const F);
        f(&StatusIcon::from_glib_borrow(this).unsafe_cast_ref())
    }
    
    unsafe extern "C" fn notify_tooltip_text_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkStatusIcon, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
    where P: IsA<StatusIcon> {
        let f: &F = &*(f as *const F);
        f(&StatusIcon::from_glib_borrow(this).unsafe_cast_ref())
    }
    
    unsafe extern "C" fn notify_visible_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkStatusIcon, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
    where P: IsA<StatusIcon> {
        let f: &F = &*(f as *const F);
        f(&StatusIcon::from_glib_borrow(this).unsafe_cast_ref())
    }
    
    impl fmt::Display for StatusIcon {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "StatusIcon")
        }
    }
    
    let f: &F = &*(f as *const F);
    f(&StatusIcon::from_glib_borrow(this).unsafe_cast_ref(), &from_glib_borrow(event)).into_glib()
}

unsafe extern "C" fn popup_menu_trampoline<P, F: Fn(&P, u32, u32) + 'static>(this: *mut gtk_sys::GtkStatusIcon, button: libc::c_uint, activate_time: libc::c_uint, f: glib_sys::gpointer)
where P: IsA<StatusIcon> {
    let f: &F = &*(f as *const F);
    f(&StatusIcon::from_glib_borrow(this).unsafe_cast_ref(), button, activate_time)
}

unsafe extern "C" fn query_tooltip_trampoline<P, F: Fn(&P, i32, i32, bool, &Tooltip) -> bool + 'static>(this: *mut gtk_sys::GtkStatusIcon, x: libc::c_int, y: libc::c_int, keyboard_mode: glib_sys::gboolean, tooltip: *mut gtk_sys::GtkTooltip, f: glib_sys::gpointer) -> glib_sys::gboolean
where P: IsA<StatusIcon> {
    let f: &F = &*(f as *const F);
    f(&StatusIcon::from_glib_borrow(this).unsafe_cast_ref(), x, y, from_glib(keyboard_mode), &from_glib_borrow(tooltip)).into_glib()
}

unsafe extern "C" fn scroll_event_trampoline<P, F: Fn(&P, &gdk::EventScroll) -> bool + 'static>(this: *mut gtk_sys::GtkStatusIcon, event: *mut gdk_sys::GdkEventScroll, f: glib_sys::gpointer) -> glib_sys::gboolean
where P: IsA<StatusIcon> {
    let f: &F = &*(f as *const F);
    f(&StatusIcon::from_glib_borrow(this).unsafe_cast_ref(), &from_glib_borrow(event)).into_glib()
}

unsafe extern "C" fn size_changed_trampoline<P, F: Fn(&P, i32) -> bool + 'static>(this: *mut gtk_sys::GtkStatusIcon, size: libc::c_int, f: glib_sys::gpointer) -> glib_sys::gboolean
where P: IsA<StatusIcon> {
    let f: &F = &*(f as *const F);
    f(&StatusIcon::from_glib_borrow(this).unsafe_cast_ref(), size).into_glib()
}

unsafe extern "C" fn notify_embedded_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkStatusIcon, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<StatusIcon> {
    let f: &F = &*(f as *const F);
    f(&StatusIcon::from_glib_borrow(this).unsafe_cast_ref())
}

unsafe extern "C" fn notify_file_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkStatusIcon, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<StatusIcon> {
    let f: &F = &*(f as *const F);
    f(&StatusIcon::from_glib_borrow(this).unsafe_cast_ref())
}

unsafe extern "C" fn notify_gicon_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkStatusIcon, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<StatusIcon> {
    let f: &F = &*(f as *const F);
    f(&StatusIcon::from_glib_borrow(this).unsafe_cast_ref())
}

unsafe extern "C" fn notify_has_tooltip_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkStatusIcon, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<StatusIcon> {
    let f: &F = &*(f as *const F);
    f(&StatusIcon::from_glib_borrow(this).unsafe_cast_ref())
}

unsafe extern "C" fn notify_icon_name_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkStatusIcon, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<StatusIcon> {
    let f: &F = &*(f as *const F);
    f(&StatusIcon::from_glib_borrow(this).unsafe_cast_ref())
}

unsafe extern "C" fn notify_orientation_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkStatusIcon, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<StatusIcon> {
    let f: &F = &*(f as *const F);
    f(&StatusIcon::from_glib_borrow(this).unsafe_cast_ref())
}

unsafe extern "C" fn notify_pixbuf_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkStatusIcon, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<StatusIcon> {
    let f: &F = &*(f as *const F);
    f(&StatusIcon::from_glib_borrow(this).unsafe_cast_ref())
}

unsafe extern "C" fn notify_screen_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkStatusIcon, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<StatusIcon> {
    let f: &F = &*(f as *const F);
    f(&StatusIcon::from_glib_borrow(this).unsafe_cast_ref())
}

unsafe extern "C" fn notify_size_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkStatusIcon, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<StatusIcon> {
    let f: &F = &*(f as *const F);
    f(&StatusIcon::from_glib_borrow(this).unsafe_cast_ref())
}

unsafe extern "C" fn notify_storage_type_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkStatusIcon, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<StatusIcon> {
    let f: &F = &*(f as *const F);
    f(&StatusIcon::from_glib_borrow(this).unsafe_cast_ref())
}

unsafe extern "C" fn notify_title_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkStatusIcon, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<StatusIcon> {
    let f: &F = &*(f as *const F);
    f(&StatusIcon::from_glib_borrow(this).unsafe_cast_ref())
}

unsafe extern "C" fn notify_tooltip_markup_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkStatusIcon, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<StatusIcon> {
    let f: &F = &*(f as *const F);
    f(&StatusIcon::from_glib_borrow(this).unsafe_cast_ref())
}

unsafe extern "C" fn notify_tooltip_text_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkStatusIcon, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<StatusIcon> {
    let f: &F = &*(f as *const F);
    f(&StatusIcon::from_glib_borrow(this).unsafe_cast_ref())
}

unsafe extern "C" fn notify_visible_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkStatusIcon, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<StatusIcon> {
    let f: &F = &*(f as *const F);
    f(&StatusIcon::from_glib_borrow(this).unsafe_cast_ref())
}

impl fmt::Display for StatusIcon {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "StatusIcon")
    }
}
