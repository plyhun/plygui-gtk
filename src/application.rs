use crate::common::{self, *};
use crate::window::Window;
use crate::tray::Tray;

use gtk::{Application as GtkApplicationSys};
use gio::ApplicationFlags;

use plygui_api::development;
use plygui_api::{controls, ids, types};

pub struct GtkApplication {
    app: GtkApplicationSys,
    name: String,
    windows: Vec<Widget>,
    trays: Vec<Object>,
    selfptr: *mut Application,
}

pub type Application = development::Application<GtkApplication>;

impl development::HasNativeIdInner for GtkApplication {
    type Id = common::GtkWidget;

    unsafe fn native_id(&self) -> Self::Id {
        self.app.clone().upcast::<Object>().into()
    }
}

impl development::ApplicationInner for GtkApplication {
    fn get() -> Box<Application> {
        if gtk::init().is_err() {
            panic!("Failed to initialize GTK");
        }
        let mut a = Box::new(development::Application::with_inner(
            GtkApplication {
                app: GtkApplicationSys::new("application.plygui", ApplicationFlags::FLAGS_NONE).unwrap(),
                name: String::new(), // TODO later // name.into(),
                windows: Vec::with_capacity(1),
                trays: Vec::with_capacity(0),
                selfptr: ptr::null_mut(),
            },
            (),
        ));
        a.as_inner_mut().selfptr = a.as_mut() as *mut Application;
        a
    }
    fn new_window(&mut self, title: &str, size: types::WindowStartSize, menu: types::Menu) -> Box<dyn controls::Window> {
        let w = super::window::GtkWindow::with_params(title, size, menu);
        let widget = {
            use plygui_api::controls::AsAny;
            let widget: Widget = unsafe { w.as_any().downcast_ref::<Window>().unwrap().as_inner().native_id().as_ref().clone().downcast().unwrap() };
            widget
        };
        self.windows.push(widget);

        w
    }
    fn new_tray(&mut self, title: &str, menu: types::Menu) -> Box<dyn controls::Tray> {
        let t = super::tray::GtkTray::with_params(title, menu);
        let o = {
            use plygui_api::controls::AsAny;
            
            let o: Object = unsafe { t.as_any().downcast_ref::<Tray>().unwrap().as_inner().native_id().as_ref().clone() };
            o
        };
        self.trays.push(o);
        t
    }
    fn remove_window(&mut self, id: Self::Id) {
        self.windows.retain(|w| GtkWidget::from(w.clone().upcast::<Object>()) != id);
    }
    fn remove_tray(&mut self, id: Self::Id) {
        self.trays.retain(|t| GtkWidget::from(t.clone()) != id);
    }
    fn name(&self) -> ::std::borrow::Cow<'_, str> {
        ::std::borrow::Cow::Borrowed(self.name.as_ref())
    }
    fn start(&mut self) {
        gtk::main()
    }
    fn exit(&mut self, skip_on_close: bool) -> bool {
        use crate::plygui_api::controls::Closeable;

        let mut n = self.windows.len() as isize;
        let mut i = n - 1;
        while i >= 0 {
            let window = &mut self.windows[i as usize];
            if let Some(window) = common::cast_gtk_widget_to_member_mut::<Window>(window) {
                if !window.close(skip_on_close) {
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
                if !tray.close(skip_on_close) {
                    return false;
                }
            }
            i -= 1;
        }
        
        gtk::main_quit();
        true
    }
    fn find_member_by_id_mut(&mut self, id: ids::Id) -> Option<&mut dyn controls::Member> {
        use plygui_api::controls::{Container, Member, SingleContainer};

        for window in self.windows.as_mut_slice() {
            let window: &mut Window = common::cast_gtk_widget_to_member_mut(window).unwrap();
            if window.id() == id {
                return Some(window.as_single_container_mut().as_container_mut().as_member_mut());
            } else {
                return window.find_control_by_id_mut(id).map(|control| control.as_member_mut());
            }
        }
        None
    }
    fn find_member_by_id(&self, id: ids::Id) -> Option<&dyn controls::Member> {
        use plygui_api::controls::{Container, Member, SingleContainer};

        for window in self.windows.as_slice() {
            let window: &Window = common::cast_gtk_widget_to_member(window).unwrap();
            if window.id() == id {
                return Some(window.as_single_container().as_container().as_member());
            } else {
                return window.find_control_by_id(id).map(|control| control.as_member());
            }
        }

        None
    }
}
