#![allow(deprecated)]

// TODO use libappindicator

use crate::common::{self, *};

use gtk::{MenuExtManual, StatusIcon as GtkStatusIcon, StatusIconExt};

#[repr(C)]
pub struct GtkTray {
    tray: GtkStatusIcon,
    context_menu: Option<GtkMenu>,
    menu: Vec<callbacks::Action>,
    on_close: Option<callbacks::OnClose>,
}

pub type Tray = AMember<ACloseable<ATray<GtkTray>>>;

impl GtkTray {
    fn set_image_inner(&mut self, i: image::DynamicImage) {
    	/*let i = {
    		let status_size = self.tray.get_size() as u32;
    		i.resize(status_size, status_size, image::FilterType::Lanczos3)
    	};*/
    	let i = common::image_to_pixbuf(&i);
    	self.tray.set_from_pixbuf(Some(&i));
    }
}
impl HasLabelInner for GtkTray {
    fn label(&self, _: &MemberBase) -> Cow<str> {
        Cow::Owned(self.tray.get_title().unwrap_or(String::new()))
    }
    fn set_label(&mut self, _: &mut MemberBase, label: Cow<str>) {
        self.tray.set_title(&label);
    }
}

impl CloseableInner for GtkTray {
    fn close(&mut self, skip_callbacks: bool) -> bool {
        if !skip_callbacks {
            let mut tray2 = self.tray.clone().upcast::<Object>();
            let tray2 = unsafe { common::cast_gobject_mut::<Tray>(&mut tray2).unwrap() };
            if let Some(ref mut on_close) = self.on_close {
                if !(on_close.as_mut())(tray2) {
                    return false;
                }
            }
        }
        self.tray.set_visible(false);
        true
    }
    fn on_close(&mut self, callback: Option<callbacks::OnClose>) {
        self.on_close = callback;
    }
    fn application<'a>(&'a self, base: &'a MemberBase) -> &'a dyn controls::Application {
        unsafe { utils::base_to_impl::<Tray>(base) }.inner().application_impl::<crate::application::Application>()
    }
    fn application_mut<'a>(&'a mut self, base: &'a mut MemberBase) -> &'a mut dyn controls::Application {
        unsafe { utils::base_to_impl_mut::<Tray>(base) }.inner_mut().application_impl_mut::<crate::application::Application>()
    }
}

impl HasImageInner for GtkTray {
	fn image(&self, _base: &MemberBase) -> Cow<image::DynamicImage> {
        unimplemented!()
    }
    fn set_image(&mut self, _base: &mut MemberBase, i: Cow<image::DynamicImage>) {
    	self.set_image_inner(i.into_owned())
    }
}
impl<O: controls::Tray> NewTrayInner<O> for GtkTray {
    fn with_uninit_params(u: &mut mem::MaybeUninit<O>, title: &str, icon: image::DynamicImage, menu: types::Menu) -> Self {
        let selfptr = u as *mut _ as *mut Tray;
        let mut t = GtkTray {
            tray: GtkStatusIcon::new_from_icon_name(title.as_ref()),
            context_menu: if menu.is_some() { Some(GtkMenu::new()) } else { None },
            menu: if menu.is_some() { Vec::new() } else { Vec::with_capacity(0) },
            on_close: None,
        };
        t.tray.set_title(title);
        t.set_image_inner(icon);
        {
            common::set_pointer(&mut t.tray.clone().upcast::<Object>(), selfptr as *mut std::os::raw::c_void);

            if let Some(menu) = menu {
                fn item_spawn(id: usize, selfptr: *mut Tray) -> GtkMenuItem {
                    let mi = GtkMenuItem::new();
                    common::set_pointer(&mut mi.clone().upcast(), selfptr as *mut std::os::raw::c_void);
                    mi.connect_activate(move |this| {
                        let mut t = this.clone().upcast::<Widget>();
                        let t = common::cast_gtk_widget_to_member_mut::<Tray>(&mut t).unwrap();
                        if let Some(a) = t.inner_mut().inner_mut().inner_mut().menu.get_mut(id) {
                            let t = unsafe { &mut *selfptr };
                            (a.as_mut())(t);
                        }
                    });
                    mi
                };

                let context_menu = t.context_menu.as_ref().unwrap();
                common::make_menu(context_menu.clone().upcast(), menu, &mut t.menu, item_spawn, selfptr);
            }
            t.tray.connect_popup_menu(popup_menu);
        }
        t
    }
}
impl TrayInner for GtkTray {
    fn with_params<S: AsRef<str>>(app: &mut dyn controls::Application, title: S, icon: image::DynamicImage, menu: types::Menu) -> Box<dyn controls::Tray> {
        let mut b: Box<mem::MaybeUninit<Tray>> = Box::new_uninit();
        let ab = AMember::with_inner(
            ACloseable::with_inner(
                ATray::with_inner(
                    <Self as NewTrayInner<Tray>>::with_uninit_params(b.as_mut(), title.as_ref(), icon, menu),
                ),
	            app.as_any_mut().downcast_mut::<crate::application::Application>().unwrap()
            )
        );
        unsafe {
	        b.as_mut_ptr().write(ab);
	        b.assume_init()
        }
    }
}

impl HasNativeIdInner for GtkTray {
    type Id = common::GtkWidget;

    fn native_id(&self) -> Self::Id {
        self.tray.clone().upcast::<Object>().into()
    }
}

impl MemberInner for GtkTray {}

fn popup_menu<'a>(this: &'a GtkStatusIcon, user_data: u32, button: u32) {
    let mut t = this.clone().upcast::<Object>();
    let this: &'static GtkStatusIcon = unsafe { mem::transmute(this) };
    let t = unsafe { common::cast_gobject_mut::<Tray>(&mut t).unwrap() };
    if let Some(ref mut menu) = t.inner_mut().inner_mut().inner_mut().context_menu {
        menu.show_all();
        menu.popup(Option::<&GtkMenu>::None, Option::<&GtkMenu>::None, move |menu, x, y| GtkStatusIcon::position_menu(menu, x, y, this), user_data, button);
    }
}
