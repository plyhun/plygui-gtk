use super::*;
use super::common::*;

use gtk::prelude::*;
use gtk::{Window as GtkWindow, WindowType};

use plygui_api::{development, ids, types, callbacks};
use plygui_api::traits::{UiControl, UiWindow, UiSingleContainer, UiMember, UiContainer, UiHasLabel};
use plygui_api::members::MEMBER_ID_WINDOW;

use std::borrow::Cow;
use std::mem;

#[repr(C)]
pub struct Window {
	base: development::UiMemberCommon,
	
    window: GtkWindow,
    
    child: Option<Box<UiControl>>,
    h_resize: Option<callbacks::Resize>,
}

impl Window {
    pub(crate) fn new(
                      title: &str,
                      start_size: types::WindowStartSize,
                      has_menu: bool)
                      -> Box<Window> {
        use gtk::GtkWindowExt;
                      	
        let mut window = Box::new(Window {
	        base: development::UiMemberCommon::with_params(
	            types::Visibility::Visible,
                development::UiMemberFunctions {
                    fn_member_id: member_id,
                    fn_is_control: is_control,
                    fn_is_control_mut: is_control_mut,
                    fn_size: size,
                }
            ),
	        window: GtkWindow::new(WindowType::Toplevel),
	        child: None,
	        h_resize: None,
        });
        window.set_label(title);
        match start_size {
	        types::WindowStartSize::Exact(w, h) => {
		        window.window.set_default_size(w as i32, h as i32);
	        }
	        types::WindowStartSize::Fullscreen => {
	        	use gdk::ScreenExt;
	        	let screen = window.window.get_screen().unwrap();
	        	window.window.set_default_size(screen.get_width(), screen.get_height());
	        }
        }
        window.window.show();
        window
    }
}

impl UiHasLabel for Window {
	fn label<'a>(&'a self) -> Cow<'a, str> {
		Cow::Owned(self.window.get_title().unwrap_or(String::new()))
	}
    fn set_label(&mut self, label: &str) {
    	self.window.set_title(label);        
    }
}

impl UiWindow for Window {
	fn as_single_container(&self) -> &UiSingleContainer {
		self
	}
	fn as_single_container_mut(&mut self) -> &mut UiSingleContainer {
		self
	}
}

impl UiSingleContainer for Window {
	fn set_child(&mut self, mut child: Option<Box<UiControl>>) -> Option<Box<UiControl>> {
		/*let mut old = self.child.take();
        if let Some(old) = old.as_mut() {
            old.on_removed_from_container(self);
        }
        if let Some(new) = child.as_mut() {
        	unsafe {
        		let mut base: &mut QtControlBase = common::cast_uicommon_to_qtcommon_mut(mem::transmute(new.as_base_mut()));		
				self.window.as_mut().set_central_widget(base.widget.as_mut_ptr());
        	}
            new.on_added_to_container(self, 0, 0);
        } else {
        	unsafe {
        		self.window.as_mut().set_central_widget(QWidget::new().into_raw());
        	}
        }
        self.child = child;

        old*/
		unimplemented!();
    }
    fn child(&self) -> Option<&UiControl> {
        //self.child.as_ref().map(|c| c.as_ref())
        unimplemented!();
    }
    fn child_mut(&mut self) -> Option<&mut UiControl> {
        /*if let Some(child) = self.child.as_mut() {
            Some(child.as_mut())
        } else {
            None
        }*/
        unimplemented!();
    }
    fn as_container(&self) -> &UiContainer {
    	self
    }
	fn as_container_mut(&mut self) -> &mut UiContainer {
		self
	}
}

impl UiContainer for Window {
    fn find_control_by_id_mut(&mut self, id_: ids::Id) -> Option<&mut UiControl> {
        /*if let Some(child) = self.child.as_mut() {
            if let Some(c) = child.is_container_mut() {
                return c.find_control_by_id_mut(id_);
            }
        }*/
        None
    }
    fn find_control_by_id(&self, id_: ids::Id) -> Option<&UiControl> {
        /*if let Some(child) = self.child.as_ref() {
            if let Some(c) = child.is_container() {
                return c.find_control_by_id(id_);
            }
        }*/
        None
    }
    fn is_single_mut(&mut self) -> Option<&mut UiSingleContainer> {
        Some(self)
    }
    fn is_single(&self) -> Option<&UiSingleContainer> {
        Some(self)
    }
    fn as_member(&self) -> &UiMember {
    	self
    }
	fn as_member_mut(&mut self) -> &mut UiMember {
		self
	}
}

impl UiMember for Window {
    fn set_visibility(&mut self, visibility: types::Visibility) {
        self.base.visibility = visibility;
        /*if types::Visibility::Visible == visibility {
            self.window.slots().set_visible();
        } else {
            self.window.slots().set_hidden();
        }*/
    }
    fn visibility(&self) -> types::Visibility {
        self.base.visibility
    }
    fn size(&self) -> (u16, u16) {
        let size = self.window.get_size();
        (size.0 as u16, size.1 as u16)
    }
    fn on_resize(&mut self, handler: Option<callbacks::Resize>) {
        self.h_resize = handler;
        
    }
	unsafe fn native_id(&self) -> usize {
    	//self.window.win_id() as usize
    	unimplemented!();
    }
    
    fn is_control_mut(&mut self) -> Option<&mut UiControl> {
    	None
    }
    fn is_control(&self) -> Option<&UiControl> {
    	None
    }
    fn as_base(&self) -> &types::UiMemberBase {
    	self.base.as_ref()
    }
    fn as_base_mut(&mut self) -> &mut types::UiMemberBase {
    	self.base.as_mut()
    }
}

unsafe fn is_control(_: &development::UiMemberCommon) -> Option<&development::UiControlCommon> {
    None
}
unsafe fn is_control_mut(_: &mut development::UiMemberCommon) -> Option<&mut development::UiControlCommon> {
    None
}
impl_size!(Window);
impl_member_id!(MEMBER_ID_WINDOW);
