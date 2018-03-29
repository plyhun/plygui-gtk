use plygui_api::{development, ids, layout, types, callbacks, traits};

use gtk::{Widget, WidgetExt};
use glib::translate::ToGlibPtr;

use std::mem;
use std::ffi::CString;
use std::os::raw::{c_char, c_void};

lazy_static! {
	pub static ref PROPERTY: CString = CString::new("plygui").unwrap();
}

#[repr(C)]
pub struct GtkControlBase {
    pub control_base: development::UiControlCommon, 
    
    pub widget: Widget,
    pub coords: Option<(i32, i32)>,
    pub measured_size: (u16, u16),
    
    pub h_resize: Option<callbacks::Resize>,
    
    invalidate: unsafe fn(this: &mut GtkControlBase),
}

impl GtkControlBase {
	pub fn with_params(widget: Widget, invalidate: unsafe fn(this: &mut GtkControlBase), functions: development::UiMemberFunctions) -> GtkControlBase {
		let base = GtkControlBase {
        	control_base: development::UiControlCommon {
	        	member_base: development::UiMemberCommon::with_params(types::Visibility::Visible, functions),
		        layout: layout::Attributes {
		            width: layout::Size::MatchParent,
					height: layout::Size::WrapContent,
					gravity: layout::gravity::CENTER_HORIZONTAL | layout::gravity::TOP,
					alignment: layout::Alignment::None,
					..
					Default::default()
	            },
        	},
        	widget: widget,
        	h_resize: None,
            coords: None,
            measured_size: (0, 0),
            
            invalidate: invalidate
        };
		base
	}
	pub fn set_pointer(&mut self, ptr: *mut c_void) {
		set_pointer(&mut self.widget, ptr)
	}
	pub fn pointer(&self) -> *mut c_void {
		pointer(&self.widget)
	}
	pub fn invalidate(&mut self) {
		unsafe { (self.invalidate)(self) }
	}
    pub fn set_visibility(&mut self, visibility: types::Visibility) {
        if self.control_base.member_base.visibility != visibility {
            self.control_base.member_base.visibility = visibility;

        }
    }
    pub fn visibility(&self) -> types::Visibility {
        self.control_base.member_base.visibility
    }
    pub fn id(&self) -> ids::Id {
        self.control_base.member_base.id
    }
    pub fn parent(&self) -> Option<&types::UiMemberBase> {
        self.widget.get_parent().map(|w| cast_gtk_widget(&w).unwrap())
    }
    pub fn parent_mut(&mut self) -> Option<&mut types::UiMemberBase> {
        self.widget.get_parent().map(|mut w| cast_gtk_widget_mut(&mut w).unwrap())
    }
    pub fn root(&self) -> Option<&types::UiMemberBase> {
        self.widget.get_toplevel().map(|w| cast_gtk_widget(&w).unwrap())
    }
    pub fn root_mut(&mut self) -> Option<&mut types::UiMemberBase> {
        self.widget.get_toplevel().map(|mut w| cast_gtk_widget_mut(&mut w).unwrap())
    }
}

#[macro_export]
macro_rules! impl_invalidate {
	($typ: ty) => {
		unsafe fn invalidate_impl(this: &mut common::GtkControlBase) {
			use plygui_api::development::UiDrawable;
			
			if let Some(mut parent_widget) = this.widget.get_parent() {
				if let Some(mparent) = common::cast_gtk_widget_to_common_mut(&mut parent_widget) {
					let (pw, ph) = mparent.size();
					let this: &mut $typ = ::std::mem::transmute(this);
					let (_,_,changed) = this.measure(pw, ph);
					this.draw(None);		
							
					if mparent.is_control().is_some() {
						let wparent: &mut common::GtkControlBase = ::std::mem::transmute(mparent);
						if changed {
							wparent.invalidate();
						} 
					}
				}
			}
		}
	}
}
pub fn set_pointer(this: &mut Widget, ptr: *mut c_void) {
	unsafe {
		::gobject_sys::g_object_set_data(this.to_glib_none().0, PROPERTY.as_ptr() as *const c_char, ptr as *mut ::libc::c_void);
    }
}
pub fn pointer(this: &Widget) -> *mut c_void {
	unsafe {
    	::gobject_sys::g_object_get_data(this.to_glib_none().0, PROPERTY.as_ptr() as *const c_char) as *mut c_void
    }
}
pub fn cast_uicommon_to_gtkcommon_mut<'a>(control: &'a mut development::UiControlCommon) -> &'a mut GtkControlBase {
	unsafe {
		mem::transmute(control)
	}
}
pub fn cast_uicommon_to_gtkcommon<'a>(control: &'a development::UiControlCommon) -> &'a GtkControlBase {
	unsafe {
		mem::transmute(control)
	}
}
fn cast_gtk_widget_mut<'a, T>(this: &mut Widget) -> Option<&'a mut T> where T: Sized {
	unsafe {
		let ptr = pointer(this);
		if !ptr.is_null() {
			Some(::std::mem::transmute(ptr))
		} else {
			None
		}
	}
}
fn cast_gtk_widget<'a, T>(this: &Widget) -> Option<&'a T> where T: Sized {
	unsafe {
		let ptr = pointer(this);
		if !ptr.is_null() {
			Some(::std::mem::transmute(ptr))
		} else {
			None
		}
	}
}
pub fn cast_gtk_widget_to_uimember_mut<'a, T>(object: &'a mut Widget) -> Option<&'a mut T> where T: traits::UiMember + Sized {
	cast_gtk_widget_mut(object)
}
pub fn cast_gtk_widget_to_uimember<'a, T>(object: &'a Widget) -> Option<&'a T> where T: traits::UiMember + Sized {
	cast_gtk_widget(object)
}
pub fn cast_gtk_widget_to_common_mut<'a>(object: &'a mut Widget) -> Option<&'a mut development::UiMemberCommon> {
	cast_gtk_widget_mut(object)
}
pub fn cast_gtk_widget_to_common<'a>(object: &'a Widget) -> Option<&'a development::UiMemberCommon> {
	cast_gtk_widget(object)
}

#[macro_export]
macro_rules! impl_is_control {
	($typ: ty) => {
		unsafe fn is_control(this: &::plygui_api::development::UiMemberCommon) -> Option<&::plygui_api::development::UiControlCommon> {
			Some(&::plygui_api::utils::base_to_impl::<$typ>(this).base.control_base)
		}
		unsafe fn is_control_mut(this: &mut ::plygui_api::development::UiMemberCommon) -> Option<&mut ::plygui_api::development::UiControlCommon> {
			Some(&mut ::plygui_api::utils::base_to_impl_mut::<$typ>(this).base.control_base)
		}
	}
}
#[macro_export]
macro_rules! impl_size {
	($typ: ty) => {
		unsafe fn size(this: &::plygui_api::development::UiMemberCommon) -> (u16, u16) {
			::plygui_api::utils::base_to_impl::<$typ>(this).size()
		}
	}
}
#[macro_export]
macro_rules! impl_member_id {
	($mem: expr) => {
		unsafe fn member_id(_: &::plygui_api::development::UiMemberCommon) -> &'static str {
			$mem
		}
	}
}
#[macro_export]
macro_rules! impl_measure {
	($typ: ty) => {
		unsafe fn measure(&mut UiMemberBase, w: u16, h: u16) -> (u16, u16, bool) {
			::plygui_api::utils::base_to_impl::<$typ>(this).measure(w, h)
		}
	}
}
#[macro_export]
macro_rules! impl_draw {
	($typ: ty) => {
		unsafe fn draw(&mut UiMemberBase, coords: Option<(i32, i32)>) {
			::plygui_api::utils::base_to_impl::<$typ>(this).draw(coords)
		}
	}
}