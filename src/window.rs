use crate::common::{self, *};

use gtk::{BoxExt, Box as GtkBox, ContainerExt, GtkWindowExt, MenuBar as GtkMenuBar, OrientableExt, Rectangle, Widget, Window as GtkWindowSys, WindowType};

#[repr(C)]
pub struct GtkWindow {
    window: GtkWindowSys,
    container: reckless::RecklessBox,
    size: (i32, i32),
    child: Option<Box<dyn controls::Control>>,
    menu_bar: Option<GtkMenuBar>,
    menu: Vec<callbacks::Action>,
    on_close: Option<callbacks::OnClose>,
    skip_callbacks: bool,
}

pub type Window = AMember<AContainer<ASingleContainer<AWindow<GtkWindow>>>>;

impl GtkWindow {
    fn size_inner(&self) -> (u16, u16) {
        let size = self.window.get_size();
        (size.0 as u16, size.1 as u16)
    }
    fn redraw(&mut self) {
        let size = self.size_inner();
        if let Some(ref mut child) = self.child {
            child.measure(size.0, size.1);
            child.draw(Some((0, 0)));
        }
    }
}

impl CloseableInner for GtkWindow {
    fn close(&mut self, skip_callbacks: bool) -> bool {
        self.skip_callbacks = skip_callbacks;
        let glib::signal::Inhibit(inhibit) = on_widget_deleted(&self.window, unsafe { &mem::zeroed() });
        if inhibit {
            false
        } else {
            self.window.destroy();
            true
        }
    }
    fn on_close(&mut self, callback: Option<callbacks::OnClose>) {
        self.on_close = callback;
    }
}

impl WindowInner for GtkWindow {
    fn with_params<S: AsRef<str>>(title: S, start_size: types::WindowStartSize, menu: types::Menu) -> Box<dyn controls::Window> {
        let mut window = Box::new(AMember::with_inner(
            AContainer::with_inner(
	            ASingleContainer::with_inner(
	                AWindow::with_inner(
	                    GtkWindow {
	                        size: (0, 0),
	                        window: GtkWindowSys::new(WindowType::Toplevel),
	                        container: reckless::RecklessBox::new(),
	                        child: None,
	                        menu_bar: if menu.is_some() { Some(GtkMenuBar::new()) } else { None },
	                        menu: if menu.is_some() { Vec::new() } else { Vec::with_capacity(0) },
	                        on_close: None,
	                        skip_callbacks: false,
	                    },
	                ),
	            )
            ),
        ));

        let selfptr = window.as_mut() as *mut Window;

        {
            let window = window.inner_mut().inner_mut().inner_mut().inner_mut();
            common::set_pointer(&mut window.window.clone().upcast::<Object>(), selfptr as *mut std::os::raw::c_void);

            if let Some(menu) = menu {
                fn item_spawn(id: usize, selfptr: *mut Window) -> GtkMenuItem {
                    let mi = GtkMenuItem::new();
                    common::set_pointer(&mut mi.clone().upcast(), selfptr as *mut std::os::raw::c_void);
                    mi.connect_activate(move |this| {
                        let mut w = this.clone().upcast::<Widget>();
                        let w = common::cast_gtk_widget_to_member_mut::<Window>(&mut w).unwrap();
                        if let Some(a) = w.inner_mut().inner_mut().inner_mut().inner_mut().menu.get_mut(id) {
                            let w = unsafe { &mut *selfptr };
                            (a.as_mut())(w);
                        }
                    });
                    mi
                };

                let menu_bar = window.menu_bar.as_ref().unwrap();
                common::make_menu(menu_bar.clone().upcast::<GtkMenuBar>().upcast(), menu, &mut window.menu, item_spawn, selfptr);
                window.container.add(menu_bar);
                menu_bar.show_all();
            }

            window.container.clone().upcast::<GtkBox>().set_orientation(GtkOrientation::Vertical);
            window.window.add(&window.container);
            window.size = match start_size {
                types::WindowStartSize::Exact(w, h) => (w as i32, h as i32),
                types::WindowStartSize::Fullscreen => {
                    use gdk::ScreenExt;
                    let screen = window.window.get_screen().unwrap();
                    (screen.get_width(), screen.get_height())
                }
            };
            window.window.set_default_size(window.size.0, window.size.1);
            window.window.connect_size_allocate(on_resize_move);
            window.window.connect_destroy(move |this| {
                super::application::Application::get()
                    .unwrap()
                    .as_any_mut()
                    .downcast_mut::<super::application::Application>()
                    .unwrap()
                    .inner_mut()
                    .remove_window(this.clone().upcast::<Object>().into());
            });
            window.window.connect_delete_event(on_widget_deleted);
            window.window.show();
            window.container.show();
        }
        
        controls::HasLabel::set_label(window.as_mut(), title.as_ref().into());
        window
    }
    fn size(&self) -> (u16, u16) {
        self.size_inner()
    }
    fn position(&self) -> (i32, i32) {
        self.window.get_position()
    }
}

impl HasLabelInner for GtkWindow {
    fn label(&self, _: &MemberBase) -> ::std::borrow::Cow<str> {
        Cow::Owned(self.window.get_title().unwrap_or(String::new()))
    }
    fn set_label(&mut self, _: &mut MemberBase, label: Cow<str>) {
        self.window.set_title(&label);
        self.redraw();
    }
}

impl SingleContainerInner for GtkWindow {
    fn set_child(&mut self, base: &mut MemberBase, mut child: Option<Box<dyn controls::Control>>) -> Option<Box<dyn controls::Control>> {
        let mut old = self.child.take();
        let index = if self.menu_bar.is_some() { 1 } else { 0 };
        if let Some(old) = old.as_mut() {
            let widget = common::cast_control_to_gtkwidget(old.as_mut());
            let widget = Object::from(widget).downcast::<Widget>().unwrap();
            self.container.remove(&widget);
            let self2 = unsafe { utils::base_to_impl_mut::<Window>(base) };
            old.on_removed_from_container(self2);
        }
        if let Some(new) = child.as_mut() {
            let widget = common::cast_control_to_gtkwidget(new.as_ref());
            self.container.add(&Object::from(widget).downcast::<Widget>().unwrap());
            let widget = common::cast_control_to_gtkwidget(new.as_ref());
            self.container.set_child_position(&Object::from(widget).downcast::<Widget>().unwrap(), index);
            let (pw, ph) = self.size();
            let self2 = unsafe { utils::base_to_impl_mut::<Window>(base) };
            new.on_added_to_container(
                self2,
                0,
                0,
                utils::coord_to_size(cmp::max(0, pw as i32 - self.window.get_margin_start() - self.window.get_margin_end())),
                utils::coord_to_size(cmp::max(0, ph as i32 - self.window.get_margin_top() - self.window.get_margin_bottom())),
            );
        }
        self.child = child;

        old
    }
    fn child(&self) -> Option<&dyn controls::Control> {
        self.child.as_ref().map(|c| c.as_ref())
    }
    fn child_mut(&mut self) -> Option<&mut dyn controls::Control> {
        if let Some(child) = self.child.as_mut() {
            Some(child.as_mut())
        } else {
            None
        }
    }
}

impl ContainerInner for GtkWindow {
    fn find_control_mut(&mut self, arg: types::FindBy) -> Option<&mut dyn controls::Control> {
        if let Some(child) = self.child.as_mut() {
            if let Some(c) = child.is_container_mut() {
                return c.find_control_mut(arg);
            }
        }
        None
    }
    fn find_control(&self, arg: types::FindBy) -> Option<&dyn controls::Control> {
        if let Some(child) = self.child.as_ref() {
            if let Some(c) = child.is_container() {
                return c.find_control(arg);
            }
        }
        None
    }
}

impl HasSizeInner for GtkWindow {
    fn on_size_set(&mut self, _: &mut MemberBase, (width, height): (u16, u16)) -> bool {
        self.window.set_default_size(width as i32, height as i32);
        true
    }
}

impl HasVisibilityInner for GtkWindow {
    fn on_visibility_set(&mut self, _: &mut MemberBase, value: types::Visibility) -> bool {
        if types::Visibility::Visible == value {
            self.window.show();
        } else {
            self.window.hide();
        }
        true
    }
}

impl HasNativeIdInner for GtkWindow {
    type Id = common::GtkWidget;

    unsafe fn native_id(&self) -> Self::Id {
        self.window.clone().upcast::<Object>().into()
    }
}

impl MemberInner for GtkWindow {}

fn on_widget_deleted<'t, 'e>(this: &'t GtkWindowSys, _: &'e gdk::Event) -> glib::signal::Inhibit {
    let mut window = this.clone().upcast::<Widget>();
    let window = common::cast_gtk_widget_to_member_mut::<Window>(&mut window);
    let mut inhibit = false;
    if let Some(window) = window {
        if !window.inner_mut().inner_mut().inner_mut().inner_mut().skip_callbacks {
            let mut window2 = window.inner_mut().inner_mut().inner_mut().inner_mut().window.clone().upcast::<Widget>();
            let window2 = common::cast_gtk_widget_to_member_mut::<Window>(&mut window2);
            if let Some(window2) = window2 {
                if let Some(ref mut on_close) = window.inner_mut().inner_mut().inner_mut().inner_mut().on_close {
                    inhibit = !(on_close.as_mut())(window2);
                }
            }
        }
    }
    glib::signal::Inhibit(inhibit)
}

fn on_resize_move(this: &GtkWindowSys, allo: &Rectangle) {
    let mut window = this.clone().upcast::<Widget>();
    let window = common::cast_gtk_widget_to_member_mut::<Window>(&mut window);
    if let Some(window) = window {
        let (width, mut height) = window.inner().inner().inner().inner().size;
        if let Some(ref menu) = window.inner().inner().inner().inner().menu_bar {
            let allo = menu.get_allocation();
            height -= allo.height;
        }
        
        if width != allo.width || height != allo.height {
            window.inner_mut().inner_mut().inner_mut().inner_mut().size = (cmp::max(0, allo.width), cmp::max(0, allo.height));
            if let Some(ref mut child) = window.inner_mut().inner_mut().inner_mut().inner_mut().child {
                child.measure(width as u16, height as u16);
                child.draw(Some((0, 0)));
            }
            window.call_on_size(width as u16, height as u16);
        }
    }
}

