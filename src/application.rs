use super::*;

use plygui_api::{types, ids, controls, development};
use plygui_api::development::HasInner;

use std::mem;

pub struct GtkApplication {
	name: String,
    windows: Vec<usize>,
}

pub type Application = development::Application<GtkApplication>;

impl development::ApplicationInner for GtkApplication {
	fn with_name(name: &str) -> Box<controls::Application> {
		if gtk::init().is_err() {
	        panic!("Failed to initialize GTK");
	    }
    	Box::new(
        	development::Application::with_inner(GtkApplication { 
        		name: name.into(),
        		windows: Vec::with_capacity(1),
	        }, ())
        )
	}
	fn new_window(&mut self, title: &str, size: types::WindowStartSize, menu: types::WindowMenu) -> Box<controls::Window> {
		use plygui_api::development::WindowInner;
		
		let w = window::GtkWindow::with_params(title, size, menu);
        self.windows.push(unsafe { w.native_id() } );
        w
	}
    fn name(&self) -> ::std::borrow::Cow<str> {
    	::std::borrow::Cow::Borrowed(self.name.as_ref())
    }
    fn start(&mut self) {
    	gtk::main()
    }
    fn find_member_by_id_mut(&mut self, id: ids::Id) -> Option<&mut controls::Member> {
    	use plygui_api::controls::{SingleContainer, Member, Container};
    	
        for window in self.windows.as_mut_slice() {
            let window: &mut window::Window = unsafe { mem::transmute(*window) };
            if window.id() == id {
                return Some(window.as_single_container_mut().as_container_mut().as_member_mut());
            } else {
                return window.find_control_by_id_mut(id).map(|control| {
                    control.as_member_mut()
                });
            }
        }
        None
    }
    fn find_member_by_id(&self, id: ids::Id) -> Option<&controls::Member> {
    	use plygui_api::controls::{SingleContainer, Member, Container};
    	
    	for window in self.windows.as_slice() {
            let window: &window::Window = unsafe { mem::transmute(*window) };
            if window.id() == id {
                return Some(window.as_single_container().as_container().as_member());
            } else {
                return window.find_control_by_id(id).map(|control| {
                    control.as_member()
                });
            }
        }

        None
    }
}

impl Drop for GtkApplication {
    fn drop(&mut self) {
    	gtk::main_quit();
        gtk::prelude::Inhibit(false);
    }
}
