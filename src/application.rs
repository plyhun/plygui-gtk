use crate::common::{self, *};

use gio::ApplicationFlags;
use glib::{self, Continue};
use gtk::Application as GtkApplicationSys;

use std::any::TypeId;

const DEFAULT_FRAME_SLEEP_MS: u32 = 10;

pub struct GtkApplication {
    app: GtkApplicationSys,
    name: String,
    selfptr: *mut Application,
    sleep: u32,
}

pub type Application = AApplication<GtkApplication>;

impl HasNativeIdInner for GtkApplication {
    type Id = common::GtkWidget;

    fn native_id(&self) -> Self::Id {
        self.app.clone().upcast::<Object>().into()
    }
}

impl GtkApplication {
    fn maybe_exit(&mut self) -> bool {
        let base = &mut unsafe { &mut *self.selfptr }.base;
        if base.windows.len() < 1 && base.trays.len() < 1 {
            gtk::main_quit();
            true
        } else {
            false
        }
    }
}

impl<O: controls::Application> NewApplicationInner<O> for GtkApplication {
    fn with_uninit_params(u: &mut mem::MaybeUninit<O>, name: &str) -> Self {
        GtkApplication {
            app: GtkApplicationSys::new("application.plygui", ApplicationFlags::FLAGS_NONE).unwrap(),
            name: name.into(),
            selfptr: u as *mut _ as *mut Application,
            sleep: DEFAULT_FRAME_SLEEP_MS,
        }
    }
}

impl ApplicationInner for GtkApplication {
    fn with_name<S: AsRef<str>>(name: S) -> Box<dyn controls::Application> {
        let mut b: Box<mem::MaybeUninit<Application>> = Box::new_uninit();
        let ab = AApplication::with_inner(
            <Self as NewApplicationInner<Application>>::with_uninit_params(b.as_mut(), name.as_ref()),
        );
        unsafe {
	        b.as_mut_ptr().write(ab);
	        b.assume_init()
        }
    }
    fn frame_sleep(&self) -> u32 {
    	self.sleep
    }
    fn set_frame_sleep(&mut self, value: u32) {
    	self.sleep = value;
    }
    fn add_root(&mut self, m: Box<dyn controls::Closeable>) -> &mut dyn controls::Member {
        let base = &mut unsafe { &mut *self.selfptr }.base;
        
        let is_window = m.as_any().type_id() == TypeId::of::<crate::window::Window>();
        let is_tray = m.as_any().type_id() == TypeId::of::<crate::tray::Tray>();
        
        if is_window {
            let i = base.windows.len();
            base.windows.push(m.into_any().downcast::<crate::window::Window>().unwrap());
            return base.windows[i].as_mut().as_member_mut();
        }
        
        if is_tray {
            let i = base.trays.len();
            base.trays.push(m.into_any().downcast::<crate::tray::Tray>().unwrap());
            return base.trays[i].as_mut().as_member_mut();
        }
        
        panic!("Unsupported Closeable: {:?}", m.as_any().type_id());
    }
    fn close_root(&mut self, arg: types::FindBy, skip_callbacks: bool) -> bool {
        let base = &mut unsafe { &mut *self.selfptr }.base;
        match arg {
            types::FindBy::Id(id) => {
                (0..base.windows.len()).into_iter().find(|i| if base.windows[*i].id() == id 
                    && base.windows[*i].as_any_mut().downcast_mut::<crate::window::Window>().unwrap().inner_mut().inner_mut().inner_mut().inner_mut().close(skip_callbacks) {
                        base.windows.remove(*i);
                        self.maybe_exit();
                        true
                    } else {
                        false
                }).is_some()
                || 
                (0..base.trays.len()).into_iter().find(|i| if base.trays[*i].id() == id 
                    && base.trays[*i].as_any_mut().downcast_mut::<crate::tray::Tray>().unwrap().inner_mut().close(skip_callbacks) {
                        base.trays.remove(*i);
                        self.maybe_exit();
                        true
                    } else {
                        false
                }).is_some()
            }
            types::FindBy::Tag(ref tag) => {
                (0..base.windows.len()).into_iter().find(|i| if base.windows[*i].tag().is_some() && base.windows[*i].tag().unwrap() == Cow::Borrowed(tag.into()) 
                    && base.windows[*i].as_any_mut().downcast_mut::<crate::window::Window>().unwrap().inner_mut().inner_mut().inner_mut().inner_mut().close(skip_callbacks) {
                        base.windows.remove(*i);
                        self.maybe_exit();
                        true
                    } else {
                        false
                }).is_some()
                || 
                (0..base.trays.len()).into_iter().find(|i| if base.trays[*i].tag().is_some() && base.trays[*i].tag().unwrap() == Cow::Borrowed(tag.into()) 
                    && base.trays[*i].as_any_mut().downcast_mut::<crate::tray::Tray>().unwrap().inner_mut().close(skip_callbacks) {
                        base.trays.remove(*i);
                        self.maybe_exit();
                        true
                    } else {
                        false
                }).is_some()
            }
        }
    }
    fn name(&self) -> Cow<str> {
        Cow::Borrowed(self.name.as_ref())
    }
    fn start(&mut self) {
        {
            let selfptr = self.selfptr as usize;
            glib::idle_add(move || unsafe {
                let mut frame_callbacks = 0;
                let a = &mut *(selfptr as *mut Application);
                while frame_callbacks < defaults::MAX_FRAME_CALLBACKS {
                    let b = &mut a.base;
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
    fn exit(&mut self) {
        let base = &mut unsafe { &mut *self.selfptr }.base;
        for mut window in base.windows.drain(..) {
            window.as_any_mut().downcast_mut::<crate::window::Window>().unwrap().inner_mut().inner_mut().inner_mut().inner_mut().close(true);
        }
        for mut tray in base.trays.drain(..) {
            tray.as_any_mut().downcast_mut::<crate::tray::Tray>().unwrap().inner_mut().close(true);
        }
        self.maybe_exit();
    }
    fn find_member_mut<'a>(&'a mut self, arg: &'a types::FindBy) -> Option<&'a mut dyn controls::Member> {
        let w = &mut unsafe {&mut *self.selfptr}.base;
        for window in w.windows.as_mut_slice() {
            match arg {
                types::FindBy::Id(id) => {
                    if window.id() == *id {
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
            let found = controls::Container::find_control_mut(window.as_mut(), arg).map(|control| control.as_member_mut());
            if found.is_some() {
                return found;
            }
        }
        for tray in w.trays.as_mut_slice() {
            let tray = &mut **tray;
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
        None
    }
    fn find_member<'a>(&'a self, arg: &'a types::FindBy) -> Option<&'a dyn controls::Member> {
        let w = &unsafe { &*self.selfptr }.base;
        for window in w.windows.as_slice() {
            match arg {
                types::FindBy::Id(id) => {
                    if window.id() == *id {
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
            let found = controls::Container::find_control(window.as_ref(), arg).map(|control| control.as_member());
            if found.is_some() {
                return found;
            }
        }
        for tray in w.trays.as_slice() {
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
        None
    }
    fn roots<'a>(&'a self) -> Box<dyn Iterator<Item = &'a (dyn controls::Member)> + 'a> {
        unsafe { &*self.selfptr }.roots()
    }
    fn roots_mut<'a>(&'a mut self) -> Box<dyn Iterator<Item = &'a mut (dyn controls::Member)> + 'a> {
        unsafe { &mut *self.selfptr }.roots_mut()
    }  
}
