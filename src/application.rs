use super::*;
use super::common::*;

use gtk::prelude::*;

//use plygui_api::members::MEMBER_ID_APPLICATION;
use plygui_api::traits::{UiWindow, UiApplication, UiMember};
use plygui_api::types::WindowStartSize;
use plygui_api::ids::Id;

use std::borrow::Cow;

pub struct Application {
    windows: Vec<usize>,
}

impl Application {
    pub fn with_name(name: &str) -> Box<Application> {
    	if gtk::init().is_err() {
	        panic!("Failed to initialize GTK");
	    }
    	Box::new(
        	Application { 
        		windows: Vec::with_capacity(1),
	        }
        )
    }
}

impl UiApplication for Application {
    fn new_window(&mut self, title: &str, size: WindowStartSize, has_menu: bool) -> Box<UiWindow> {
        let mut w = Window::new(title, size, has_menu);
        //self.windows.push(w.qwindow());
        w
    }
    fn name<'a>(&'a self) -> Cow<'a, str> {
        unimplemented!()
    }
    fn start(&mut self) {
        gtk::main();
    }
    fn find_member_by_id_mut(&mut self, id: Id) -> Option<&mut UiMember> {
    	unimplemented!()
    }
    fn find_member_by_id(&self, id: Id) -> Option<&UiMember> {
    	unimplemented!()
    }
}

impl Drop for Application {
    fn drop(&mut self) {
    	gtk::main_quit();
        Inhibit(false);
    }
}
