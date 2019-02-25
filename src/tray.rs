#![allow(deprecated)]

use super::common::*;
use super::*;

use gtk::{StatusIcon, StatusIconExt};

#[repr(C)]
pub struct GtkTray {
    tray: StatusIcon,
    on_close: Option<callbacks::Action>,
    skip_callbacks: bool,
}

pub type Tray = Member<GtkTray>;

impl HasLabelInner for GtkTray {
    fn label(&self) -> ::std::borrow::Cow<'_, str> {
        Cow::Owned(self.tray.get_title().unwrap_or(String::new()))
    }
    fn set_label(&mut self, _: &mut MemberBase, label: &str) {
        self.tray.set_title(label);    
    }
}

impl CloseableInner for GtkTray {
    fn close(&mut self, skip_callbacks: bool) {
        self.skip_callbacks = skip_callbacks;
        
        self.tray.set_visible(false);
    }
    fn on_close(&mut self, callback: Option<callbacks::Action>) {
        self.on_close = callback;
    }
}

impl TrayInner for GtkTray {
    fn with_params(title: &str, _menu: types::Menu) -> Box<Member<Self>> {
        use plygui_api::controls::HasLabel; 
        
        let mut t = Box::new(Member::with_inner(
            GtkTray {
                tray: StatusIcon::new_from_icon_name(title),
                on_close: None,
                skip_callbacks: false,
            },
            MemberFunctions::new(_as_any, _as_any_mut, _as_member, _as_member_mut),
        ));
        t.set_label(title);
        t
    }
}

impl HasNativeIdInner for GtkTray {
    type Id = common::GtkWidget;

    unsafe fn native_id(&self) -> Self::Id {
        self.tray.clone().upcast::<Object>().into()
    }
}

impl MemberInner for GtkTray {}

impl_all_defaults!(Tray);
