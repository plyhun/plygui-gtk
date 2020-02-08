use crate::common::{self, *};
use crate::tray::Tray;
use crate::window::Window;

use gio::ApplicationFlags;
use glib::{self, Continue};
use gtk::Application as GtkApplicationSys;

use plygui_api::development;
use plygui_api::{controls, types};

const DEFAULT_FRAME_SLEEP_MS: u32 = 10;

pub struct GtkApplication {
    app: GtkApplicationSys,
    name: String,
    windows: Vec<Widget>,
    trays: Vec<Object>,
    selfptr: *mut Application,
    sleep: u32,
}

pub type Application = AApplication<GtkApplication>;

impl development::HasNativeIdInner for GtkApplication {
    type Id = common::GtkWidget;

    unsafe fn native_id(&self) -> Self::Id {
        self.app.clone().upcast::<Object>().into()
    }
}

impl GtkApplication {
    fn maybe_exit(&mut self) -> bool {
        if self.windows.len() < 1 && self.trays.len() < 1 {
            gtk::main_quit();
            true
        } else {
            false
        }
    }
}

impl development::ApplicationInner for GtkApplication {
    fn get() -> Box<dyn controls::Application> {
        if gtk::init().is_err() {
            panic!("Failed to initialize GTK");
        }
        let mut a = Box::new(AApplication::with_inner(
            GtkApplication {
                app: GtkApplicationSys::new("application.plygui", ApplicationFlags::FLAGS_NONE).unwrap(),
                name: String::new(), // TODO later // name.into(),
                windows: Vec::with_capacity(1),
                trays: Vec::with_capacity(0),
                selfptr: ptr::null_mut(),
                sleep: DEFAULT_FRAME_SLEEP_MS,
            },
        ));
        a.inner_mut().selfptr = a.as_mut() as *mut Application;
        a
    }
    fn frame_sleep(&self) -> u32 {
    	self.sleep
    }
    fn set_frame_sleep(&mut self, value: u32) {
    	self.sleep = value;
    }
    fn new_window(&mut self, title: &str, size: types::WindowStartSize, menu: types::Menu) -> Box<dyn controls::Window> {
        let w = super::window::GtkWindow::with_params(title, size, menu);
        let widget = unsafe { w.as_any().downcast_ref::<Window>().unwrap().inner().native_id().as_ref().clone().downcast().unwrap() };
        self.windows.push(widget);

        w
    }
    fn new_tray(&mut self, title: &str, menu: types::Menu) -> Box<dyn controls::Tray> {
        let t = super::tray::GtkTray::with_params(title, menu);
        let o = {
            let o: Object = unsafe { t.as_any().downcast_ref::<Tray>().unwrap().inner().native_id().as_ref().clone() };
            o
        };
        self.trays.push(o);
        t
    }
    fn remove_window(&mut self, id: Self::Id) {
        self.windows.retain(|w| GtkWidget::from(w.clone().upcast::<Object>()) != id);
        self.maybe_exit();
    }
    fn remove_tray(&mut self, id: Self::Id) {
        self.trays.retain(|t| GtkWidget::from(t.clone()) != id);
        self.maybe_exit();
    }
    fn name(&self) -> ::std::borrow::Cow<'_, str> {
        ::std::borrow::Cow::Borrowed(self.name.as_ref())
    }
    fn start(&mut self) {
        {
            let selfptr = self.selfptr as usize;
            glib::idle_add(move || unsafe {
                let mut frame_callbacks = 0;
                let a = &mut *(selfptr as *mut Application);
                while frame_callbacks < defaults::MAX_FRAME_CALLBACKS {
                    let b = a.base_mut();
                    match b.queue().try_recv() {
                        Ok(mut cmd) => {
                            if (cmd.as_mut())(&mut *(selfptr as *mut Application)) {
                                let _ = b.sender().send(cmd);
                            }
                            frame_callbacks += 1;
                        }
                        Err(e) => match e {
                            mpsc::TryRecvError::Empty => break,
                            mpsc::TryRecvError::Disconnected => unreachable!(),
                        },
                    }
                }
                glib::usleep(a.inner().sleep as u64);
                Continue(true)
            });
        }
        gtk::main()
    }
    fn exit(&mut self, skip_on_close: bool) -> bool {
        let mut n = self.windows.len() as isize;
        let mut i = n - 1;
        while i >= 0 {
            let window = &mut self.windows[i as usize];
            if let Some(window) = common::cast_gtk_widget_to_member_mut::<Window>(window) {
                if !controls::Closeable::close(window, skip_on_close) {
                    return false;
                }
            }
            i -= 1;
        }

        n = self.trays.len() as isize;
        i = n - 1;
        while i >= 0 {
            let tray = &mut self.trays[i as usize];
            if let Some(tray) = unsafe { common::cast_gobject_mut::<Tray>(tray) } {
                if !controls::Closeable::close(tray, skip_on_close) {
                    return false;
                }
            }
            i -= 1;
        }
        self.maybe_exit()
    }
    fn find_member_mut(&mut self, arg: types::FindBy) -> Option<&mut dyn controls::Member> {
        use crate::plygui_api::controls::{Member};

        for window in self.windows.as_mut_slice() {
            if let Some(window) = common::cast_gtk_widget_to_member_mut::<Window>(window) {
                match arg {
                    types::FindBy::Id(id) => {
                        if window.id() == id {
                            return Some(window.as_member_mut());
                        }
                    }
                    types::FindBy::Tag(ref tag) => {
                        if let Some(mytag) = window.tag() {
                            if tag.as_str() == mytag {
                                return Some(window.as_member_mut());
                            }
                        }
                    }
                }
                let found = controls::Container::find_control_mut(window, arg.clone()).map(|control| control.as_member_mut());
                if found.is_some() {
                    return found;
                }
            }
        }
        for tray in self.trays.as_mut_slice() {
            if let Some(tray) = unsafe { common::cast_gobject_mut::<Tray>(tray) } {
                match arg {
                    types::FindBy::Id(ref id) => {
                        if tray.id() == *id {
                            return Some(tray.as_member_mut());
                        }
                    }
                    types::FindBy::Tag(ref tag) => {
                        if let Some(mytag) = tray.tag() {
                            if tag.as_str() == mytag {
                                return Some(tray.as_member_mut());
                            }
                        }
                    }
                }
            }
        }
        None
    }
    fn find_member(&self, arg: types::FindBy) -> Option<&dyn controls::Member> {
        use crate::plygui_api::controls::{Member};

        for window in self.windows.as_slice() {
            if let Some(window) = common::cast_gtk_widget_to_member::<Window>(window) {
                match arg {
                    types::FindBy::Id(id) => {
                        if window.id() == id {
                            return Some(window.as_member());
                        }
                    }
                    types::FindBy::Tag(ref tag) => {
                        if let Some(mytag) = window.tag() {
                            if tag.as_str() == mytag {
                                return Some(window.as_member());
                            }
                        }
                    }
                }
                let found = controls::Container::find_control(window, arg.clone()).map(|control| control.as_member());
                if found.is_some() {
                    return found;
                }
            }
        }
        for tray in self.trays.as_slice() {
            if let Some(tray) = unsafe { common::cast_gobject::<Tray>(tray) } {
                match arg {
                    types::FindBy::Id(ref id) => {
                        if tray.id() == *id {
                            return Some(tray.as_member());
                        }
                    }
                    types::FindBy::Tag(ref tag) => {
                        if let Some(mytag) = tray.tag() {
                            if tag.as_str() == mytag {
                                return Some(tray.as_member());
                            }
                        }
                    }
                }
            }
        }
        None
    }
    fn members<'a>(&'a self) -> Box<dyn Iterator<Item = &'a (dyn controls::Member)> + 'a> {
        Box::new(MemberIterator {
            inner: self,
            is_tray: false,
            index: 0,
            needs_window: true,
            needs_tray: true,
        })
    }
    fn members_mut<'a>(&'a mut self) -> Box<dyn Iterator<Item = &'a mut (dyn controls::Member)> + 'a> {
        Box::new(MemberIteratorMut {
            inner: self,
            is_tray: false,
            index: 0,
            needs_window: true,
            needs_tray: true,
        })
    }    
}
struct MemberIterator<'a> {
    inner: &'a GtkApplication,
    needs_window: bool,
    needs_tray: bool,
    is_tray: bool,
    index: usize,
}
impl<'a> Iterator for MemberIterator<'a> {
    type Item = &'a (dyn controls::Member + 'static);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.inner.windows.len() {
            self.is_tray = true;
            self.index = 0;
        }
        let ret = if self.needs_tray && self.is_tray {
            self.inner.trays.get(self.index).map(|tray| unsafe { common::cast_gobject::<Tray>(tray).unwrap() } as &dyn controls::Member)
        } else if self.needs_window {
            self.inner.windows.get(self.index).map(|window| common::cast_gtk_widget_to_member::<Window>(window).unwrap() as &dyn controls::Member)
        } else {
            return None;
        };
        self.index += 1;
        ret
    }
}

struct MemberIteratorMut<'a> {
    inner: &'a mut GtkApplication,
    needs_window: bool,
    needs_tray: bool,
    is_tray: bool,
    index: usize,
}
impl<'a> Iterator for MemberIteratorMut<'a> {
    type Item = &'a mut (dyn controls::Member + 'static);

    fn next(&mut self) -> Option<Self::Item> {
        if self.needs_tray && self.index >= self.inner.windows.len() {
            self.is_tray = true;
            self.index = 0;
        }
        let ret = if self.needs_tray && self.is_tray {
            self.inner.trays.get_mut(self.index).map(|tray| unsafe { common::cast_gobject_mut::<Tray>(tray).unwrap() } as &mut dyn controls::Member)
        } else if self.needs_window {
            self.inner.windows.get_mut(self.index).map(|window| common::cast_gtk_widget_to_member_mut::<Window>(window).unwrap() as &mut dyn controls::Member)
        } else {
            return None;
        };
        self.index += 1;
        ret
    }
}

