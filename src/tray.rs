#![allow(deprecated)]

use crate::common::{self, *};

use gtk::{StatusIcon as GtkStatusIcon, StatusIconExt, GtkMenuExtManual};
use glib::translate::{FromGlib, FromGlibPtrFull};

#[repr(C)]
pub struct GtkTray {
    tray: GtkStatusIcon,
    context_menu: Option<GtkMenu>,
    menu: Vec<callbacks::Action>,
    on_close: Option<callbacks::Action>,
    skip_callbacks: bool,
}

pub type Tray = Member<GtkTray>;

impl HasLabelInner for GtkTray {
    fn label(&self) -> ::std::borrow::Cow<'_, str> {
        Cow::Owned(self.tray.get_property_title().map(|s| s.into()).unwrap_or(String::new()))
    }
    fn set_label(&mut self, _: &mut MemberBase, label: &str) {
        self.tray.set_property_title(label);    
    }
}

impl CloseableInner for GtkTray {
    fn close(&mut self, skip_callbacks: bool) -> bool {
        self.skip_callbacks = skip_callbacks;
        
        self.tray.set_property_visible(false);
        true
    }
    fn on_close(&mut self, callback: Option<callbacks::Action>) {
        self.on_close = callback;
    }
}

impl TrayInner for GtkTray {
    fn with_params(title: &str, menu: types::Menu) -> Box<Member<Self>> {
        use plygui_api::controls::HasLabel; 
        
        let mut tray = Box::new(Member::with_inner(
            GtkTray {
                tray: unsafe {
                    GtkStatusIcon::from_glib_full(gtk_sys::gtk_status_icon_new())
                },
                context_menu: if menu.is_some() { Some(GtkMenu::new()) } else { None },
                menu: if menu.is_some() { Vec::new() } else { Vec::with_capacity(0) },
                on_close: None,
                skip_callbacks: false,
            },
            MemberFunctions::new(_as_any, _as_any_mut, _as_member, _as_member_mut),
        ));
        
        let selfptr = tray.as_mut() as *mut Tray;
        {
            let tray = tray.as_inner_mut();
            common::set_pointer(&mut tray.tray.clone().upcast::<Object>(), selfptr as *mut std::os::raw::c_void);
            
            if let Some(menu) = menu {
                fn item_spawn(id: usize, selfptr: *mut Tray) -> GtkMenuItem {
                    let mi = GtkMenuItem::new();
                    common::set_pointer(&mut mi.clone().upcast(), selfptr as *mut std::os::raw::c_void);
                    mi.connect_activate(move |this| {
                        let mut t = this.clone().upcast::<Widget>();
                        let t = common::cast_gtk_widget_to_member_mut::<Tray>(&mut t).unwrap();
                        if let Some(a) = t.as_inner_mut().menu.get_mut(id) {
                            let t = unsafe {&mut *selfptr};
                            (a.as_mut())(t);
                        }
                    });
                    mi
                }; 
                
                let context_menu = tray.context_menu.as_ref().unwrap();
                common::make_menu(context_menu.clone().upcast(), menu, &mut tray.menu, item_spawn, selfptr);
            }
            tray.tray.connect_popup_menu(popup_menu);
            tray.tray.set_property_visible(true);
        }
        
        tray.set_label(title);
        tray
    }
}

impl HasNativeIdInner for GtkTray {
    type Id = common::GtkWidget;

    unsafe fn native_id(&self) -> Self::Id {
        self.tray.clone().upcast::<Object>().into()
    }
}

impl MemberInner for GtkTray {}

fn popup_menu<'a>(this: &'a GtkStatusIcon, user_data: u32, button: u32) {
    let mut t = this.clone().upcast::<Object>();
    //let this: &'static GtkStatusIcon = unsafe { mem::transmute(this) };
    let this = this.to_glib_none().0;
    let t = unsafe { common::cast_gobject_mut::<Tray>(&mut t).unwrap() };
    if let Some(ref mut menu) = t.as_inner_mut().context_menu {
        menu.show_all();
        menu.popup(Option::<&GtkMenu>::None, Option::<&GtkMenu>::None, move |menu, x, y| unsafe {
                let mut push_in = mem::uninitialized();
                gtk_sys::gtk_status_icon_position_menu(menu.to_glib_none().0, x, y, &mut push_in, this);
                bool::from_glib(push_in)
            }, user_data, button);
    }
}

default_impls_as!(Tray);
