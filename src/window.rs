use super::*;

use gtk::prelude::*;
use gtk::{Window as GtkWindowSys, WindowType, ResizeMode, Fixed, Rectangle, Widget};

use plygui_api::{development, ids, types, layout, controls, utils};
use plygui_api::development::HasInner;

use std::borrow::Cow;

#[repr(C)]
pub struct GtkWindow {
	window: GtkWindowSys,
    frame: reckless::RecklessFixed,
    
    gravity_horizontal: layout::Gravity,
    gravity_vertical: layout::Gravity,
    size: (i32, i32),
    
    child: Option<Box<controls::Control>>,
}

pub type Window = development::Member<development::SingleContainer<GtkWindow>>;

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

impl development::WindowInner for GtkWindow {
	fn with_params(title: &str, start_size: types::WindowStartSize, menu: types::WindowMenu) -> Box<controls::Window> {
		use plygui_api::controls::HasLabel;
		
		let mut window = Box::new(development::Member::with_inner(
				development::SingleContainer::with_inner(GtkWindow {
					size: (0, 0),
			        window: GtkWindowSys::new(WindowType::Toplevel),
			        frame: reckless::RecklessFixed::new(),
			        gravity_horizontal: Default::default(),
				    gravity_vertical: Default::default(),
				    child: None,
				}, ()), 
				development::MemberFunctions::new(_as_any, _as_any_mut, _as_member, _as_member_mut
		)));
		
		let ptr = window.as_ref() as *const _ as *mut std::os::raw::c_void;
        
        {
        	let window = window.as_inner_mut().as_inner_mut();
        	common::set_pointer(&mut window.window.clone().upcast::<Widget>(), ptr);
        
	        window.window.add(&window.frame);
	        window.size = match start_size {
		        types::WindowStartSize::Exact(w, h) => {
			        (w as i32, h as i32)
		        }
		        types::WindowStartSize::Fullscreen => {
		        	use gdk::ScreenExt;
		        	let screen = window.window.get_screen().unwrap();
		        	(screen.get_width(), screen.get_height())
		        }
	        };
	        window.window.set_default_size(window.size.0, window.size.1);
	        window.window.connect_size_allocate(on_resize_move);
	        window.window.show();
	        window.frame.show();
	        window.window.set_resize_mode(ResizeMode::Immediate);
		}
        window.set_label(title);
	    window
	}
}

impl development::HasLabelInner for GtkWindow {
	fn label(&self) -> ::std::borrow::Cow<str> {
		Cow::Owned(self.window.get_title().unwrap_or(String::new()))
	}
    fn set_label(&mut self, _: &mut development::MemberBase, label: &str) {
    	self.window.set_title(label);
    	self.redraw();
    }
}

impl development::SingleContainerInner for GtkWindow {
	fn set_child(&mut self, base: &mut development::MemberBase, mut child: Option<Box<controls::Control>>) -> Option<Box<controls::Control>> {
		let mut old = self.child.take();
        if let Some(old) = old.as_mut() {
            for child in self.frame.get_children().as_slice() {
        		self.frame.remove(child);
    		}
            let self2 = unsafe { utils::base_to_impl_mut::<Window>(base) };
            old.on_removed_from_container(self2);
        }
        if let Some(new) = child.as_mut() {
        	let widget = common::cast_control_to_gtkwidget(new.as_ref());
    		let widget: &Widget = &widget;
    		self.window.get_child().unwrap().downcast::<Fixed>().unwrap().add(widget);
            let self2 = unsafe { utils::base_to_impl_mut::<Window>(base) };
            new.on_added_to_container(self2, 0, 0);
        } 
        self.child = child;

        old
	}
    fn child(&self) -> Option<&controls::Control> {
    	self.child.as_ref().map(|c| c.as_ref())
    }
    fn child_mut(&mut self) -> Option<&mut controls::Control> {
    	if let Some(child) = self.child.as_mut() {
            Some(child.as_mut())
        } else {
            None
        }
    }
}

impl development::ContainerInner for GtkWindow {
	fn find_control_by_id_mut(&mut self, id_: ids::Id) -> Option<&mut controls::Control> {
        if let Some(child) = self.child.as_mut() {
            if let Some(c) = child.is_container_mut() {
                return c.find_control_by_id_mut(id_);
            }
        }
        None
    }
    fn find_control_by_id(&self, id_: ids::Id) -> Option<&controls::Control> {
        if let Some(child) = self.child.as_ref() {
            if let Some(c) = child.is_container() {
                return c.find_control_by_id(id_);
            }
        }
        None
    }
    fn gravity(&self) -> (layout::Gravity, layout::Gravity) {
    	(self.gravity_horizontal, self.gravity_vertical)
    }
    fn set_gravity(&mut self, _: &mut development::MemberBase, w: layout::Gravity, h: layout::Gravity) {
    	if self.gravity_horizontal != w || self.gravity_vertical != h {
    		self.gravity_horizontal = w;
    		self.gravity_vertical = h;
    		self.redraw();
    	}
    }
}

impl development::MemberInner for GtkWindow {
	type Id = common::GtkWidget;
	
    fn size(&self) -> (u16, u16) {
    	self.size_inner()
    }
    
    fn on_set_visibility(&mut self, base: &mut development::MemberBase) {
    	if types::Visibility::Visible == base.visibility {
            self.window.show();
        } else {
            self.window.hide();
        }
    }
    
    unsafe fn native_id(&self) -> Self::Id {
    	self.window.clone().upcast::<Widget>().into()
    }
}

fn on_resize_move(this: &GtkWindowSys, allo: &Rectangle) {
	let mut window = this.clone().upcast::<Widget>();
	let window = common::cast_gtk_widget_to_member_mut::<Window>(&mut window);
	if let Some(window) = window {
		let (width, height) = window.as_inner().as_inner().size;
		if width != allo.width || height != allo.height {
			use std::cmp::max;
			
			window.as_inner_mut().as_inner_mut().size = (max(0, allo.width), max(0, allo.height));
			if let Some(ref mut child) = window.as_inner_mut().as_inner_mut().child {
	            child.measure(width as u16, height as u16);
	            child.draw(Some((0, 0)));
	        }
			if let Some(ref mut cb) = window.base_mut().handler_resize {
	            let mut w2 = this.clone().upcast::<Widget>();
				let mut w2 = common::cast_gtk_widget_to_member_mut::<Window>(&mut w2).unwrap();
				(cb.as_mut())(w2, width as u16, height as u16);
	        }
		}
	}
}
impl_all_defaults!(Window);
