use crate::common::{self, *};

use glib::{self, Continue};
use gtk::{Box as GtkBox, Rectangle, Widget, Window as GtkWindowSys, GtkWindowExt, WindowType, ContainerExt, BinExt, OrientableExt, MenuBar as GtkMenuBar};

#[repr(C)]
pub struct GtkWindow {
    window: GtkWindowSys,
    container: reckless::boxc::RecklessBox,
    size: (i32, i32),
    child: Option<Box<dyn controls::Control>>,
    menu_bar: Option<GtkMenuBar>,
    menu: Vec<callbacks::Action>,
    on_close: Option<callbacks::Action>,
    skip_callbacks: bool,
}

pub type Window = Member<SingleContainer<::plygui_api::development::Window<GtkWindow>>>;

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
        self.window.close();
        true
    }
    fn on_close(&mut self, callback: Option<callbacks::Action>) {
        self.on_close = callback;
    }
}

impl WindowInner for GtkWindow {
    fn with_params(title: &str, start_size: types::WindowStartSize, menu: types::Menu) -> Box<Window> {
        use plygui_api::controls::HasLabel;

        let mut window = Box::new(Member::with_inner(
            SingleContainer::with_inner(
                ::plygui_api::development::Window::with_inner(
                    GtkWindow {
                        size: (0, 0),
                        window: GtkWindowSys::new(WindowType::Toplevel),
                        container: reckless::boxc::RecklessBox::new(),
                        child: None,
                        menu_bar: if menu.is_some() { Some(GtkMenuBar::new()) } else { None },
                        menu: if menu.is_some() { Vec::new() } else { Vec::with_capacity(0) },
                        on_close: None,
                        skip_callbacks: false,
                    },
                    (),
                ),
                (),
            ),
            MemberFunctions::new(_as_any, _as_any_mut, _as_member, _as_member_mut),
        ));

        let selfptr = window.as_mut() as *mut Window;

        {
            let window = window.as_inner_mut().as_inner_mut().as_inner_mut();
            common::set_pointer(&mut window.window.clone().upcast::<Object>(), selfptr as *mut std::os::raw::c_void);
            
            if let Some(menu) = menu {
                fn item_spawn(id: usize, selfptr: *mut Window) -> GtkMenuItem {
                    let mi = GtkMenuItem::new();
                    common::set_pointer(&mut mi.clone().upcast(), selfptr as *mut std::os::raw::c_void);
                    mi.connect_activate(move |this| {
                        let mut w = this.clone().upcast::<Widget>();
                        let w = common::cast_gtk_widget_to_member_mut::<Window>(&mut w).unwrap();
                        if let Some(a) = w.as_inner_mut().as_inner_mut().as_inner_mut().menu.get_mut(id) {
                            let w = unsafe {&mut *selfptr};
                            (a.as_mut())(w);
                        }
                    });
                    mi
                }; 
                
                let menu_bar = window.menu_bar.as_ref().unwrap();
                common::make_menu(menu_bar.clone().upcast(), menu, &mut window.menu, item_spawn, selfptr);
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
            window.window.connect_delete_event(on_widget_deleted);
            window.window.show();
            window.container.show();
        }
        {
            let window = window.as_inner_mut().as_inner_mut().as_inner_mut();
            let mut window = window.window.clone().upcast::<Widget>();
            let selfptr = cast_gtk_widget_to_member_mut::<Window>(&mut window).unwrap() as *mut Window as usize;
            glib::idle_add(move || unsafe {
                let mut frame_callbacks = 0;
                while frame_callbacks < defaults::MAX_FRAME_CALLBACKS {
                    let w = (&mut *(selfptr as *mut Window)).as_inner_mut().as_inner_mut().base_mut();
                    match w.queue().try_recv() {
                        Ok(mut cmd) => {
                            if (cmd.as_mut())(&mut *(selfptr as *mut Window)) {
                                let _ = w.sender().send(cmd);
                            }
                            frame_callbacks += 1;
                        }
                        Err(e) => match e {
                            mpsc::TryRecvError::Empty => break,
                            mpsc::TryRecvError::Disconnected => unreachable!(),
                        },
                    }
                }
                glib::usleep(100);
                Continue(true)
            });
        }
        window.set_label(title);
        window
    }
    fn on_frame(&mut self, cb: callbacks::OnFrame) {
        let mut window = self.window.clone().upcast::<Widget>();
        let _ = cast_gtk_widget_to_member_mut::<Window>(&mut window).unwrap().as_inner_mut().as_inner_mut().base_mut().sender().send(cb);
    }
    fn on_frame_async_feeder(&mut self) -> callbacks::AsyncFeeder<callbacks::OnFrame> {
        let mut window = self.window.clone().upcast::<Widget>();
        cast_gtk_widget_to_member_mut::<Window>(&mut window).unwrap().as_inner_mut().as_inner_mut().base_mut().sender().clone().into()
    }
    fn size(&self) -> (u16, u16) {
        self.size_inner()
    }
    fn position(&self) -> (i32, i32) {
        self.window.get_position()
    }
}

impl HasLabelInner for GtkWindow {
    fn label(&self) -> ::std::borrow::Cow<'_, str> {
        Cow::Owned(self.window.get_title().unwrap_or(String::new()))
    }
    fn set_label(&mut self, _: &mut MemberBase, label: &str) {
        self.window.set_title(label);
        self.redraw();
    }
}

impl SingleContainerInner for GtkWindow {
    fn set_child(&mut self, base: &mut MemberBase, mut child: Option<Box<dyn controls::Control>>) -> Option<Box<dyn controls::Control>> {
        let mut old = self.child.take();
        if let Some(old) = old.as_mut() {
            for child in self.container.get_children().as_slice() {
                self.container.remove(child);
            }
            let self2 = unsafe { utils::base_to_impl_mut::<Window>(base) };
            old.on_removed_from_container(self2);
        }
        if let Some(new) = child.as_mut() {
            let widget = common::cast_control_to_gtkwidget(new.as_ref());
            self.window.get_child().unwrap().downcast::<GtkBox>().unwrap().add(&Object::from(widget).downcast::<Widget>().unwrap());
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
    fn find_control_by_id_mut(&mut self, id_: ids::Id) -> Option<&mut dyn controls::Control> {
        if let Some(child) = self.child.as_mut() {
            if let Some(c) = child.is_container_mut() {
                return c.find_control_by_id_mut(id_);
            }
        }
        None
    }
    fn find_control_by_id(&self, id_: ids::Id) -> Option<&dyn controls::Control> {
        if let Some(child) = self.child.as_ref() {
            if let Some(c) = child.is_container() {
                return c.find_control_by_id(id_);
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
    if let Some(window) = window {
        if !window.as_inner_mut().as_inner_mut().as_inner_mut().skip_callbacks {
            let mut window2 = window.as_inner_mut().as_inner_mut().as_inner_mut().window.clone().upcast::<Widget>();
            let window2 = common::cast_gtk_widget_to_member_mut::<Window>(&mut window2);
            if let Some(window2) = window2 {
                if let Some(ref mut on_close) = window.as_inner_mut().as_inner_mut().as_inner_mut().on_close {
                    if !(on_close.as_mut())(window2) {
                        return glib::signal::Inhibit(true);
                    }
                }
            }
        }
    }
    glib::signal::Inhibit(false)
}

fn on_resize_move(this: &GtkWindowSys, allo: &Rectangle) {
    let mut window = this.clone().upcast::<Widget>();
    let window = common::cast_gtk_widget_to_member_mut::<Window>(&mut window);
    if let Some(window) = window {
        let (width, height) = window.as_inner().as_inner().as_inner().size;
        if width != allo.width || height != allo.height {
            use std::cmp::max;

            window.as_inner_mut().as_inner_mut().as_inner_mut().size = (max(0, allo.width), max(0, allo.height));
            if let Some(ref mut child) = window.as_inner_mut().as_inner_mut().as_inner_mut().child {
                child.measure(width as u16, height as u16);
                child.draw(Some((0, 0)));
            }
            window.call_on_size(width as u16, height as u16);
        }
    }
}
default_impls_as!(Window);
