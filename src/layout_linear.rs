use super::*;

use plygui_api::{layout, ids, types, development, callbacks};
use plygui_api::traits::{UiControl, UiHasLayout, UiMultiContainer, UiLinearLayout, UiMember, UiContainer, UiHasOrientation};
use plygui_api::members::MEMBER_ID_LAYOUT_LINEAR;

use gtk::{Cast, Widget, WidgetExt, Fixed, FixedExt, ContainerExt, Rectangle};

use std::mem;
use std::os::raw::c_void;

const DEFAULT_PADDING: i32 = 0;

#[repr(C)]
pub struct LinearLayout {
    base: common::GtkControlBase,
    orientation: layout::Orientation,
    children: Vec<Box<UiControl>>,
}

impl LinearLayout {
    pub fn new(orientation: layout::Orientation) -> Box<LinearLayout> {
        let mut ll = Box::new(LinearLayout {
                     base: common::GtkControlBase::with_params(
                     	reckless::RecklessFixed::new().upcast::<Widget>(),
                     	invalidate_impl,
                     	development::UiMemberFunctions {
                             fn_member_id: member_id,
						     fn_is_control: is_control,
						     fn_is_control_mut: is_control_mut,
						     fn_size: size,
                        },
                     ),
                     orientation: orientation,
                     children: Vec::new(),
                 });
        {
        	let ptr = ll.as_ref() as *const _ as *mut std::os::raw::c_void;
        	ll.base.set_pointer(ptr);
        }
        ll.set_layout_padding(layout::BoundarySize::AllTheSame(DEFAULT_PADDING).into());
        ll.base.widget.connect_size_allocate(on_resize_move);
        ll
    }
}

impl UiMember for LinearLayout {
    fn set_visibility(&mut self, visibility: types::Visibility) {
        self.base.set_visibility(visibility);
    }
    fn visibility(&self) -> types::Visibility {
        self.base.visibility()
    }
    fn size(&self) -> (u16, u16) {
        self.base.measured_size
    }
    fn on_resize(&mut self, handler: Option<callbacks::Resize>) {
        self.base.h_resize = handler;
    }
	
    unsafe fn native_id(&self) -> usize {
        self.base.pointer() as usize
    }
    fn is_control(&self) -> Option<&UiControl> {
    	Some(self)
    }
    fn is_control_mut(&mut self) -> Option<&mut UiControl> {
    	Some(self)
    } 
    fn as_base(&self) -> &types::UiMemberBase {
    	self.base.control_base.member_base.as_ref()
    }
    fn as_base_mut(&mut self) -> &mut types::UiMemberBase {
    	self.base.control_base.member_base.as_mut()
    }
}

impl development::UiDrawable for LinearLayout {
	fn draw(&mut self, coords: Option<(i32, i32)>) {
    	if coords.is_some() {
    		self.base.coords = coords;
    	}
    	if let Some((x, y)) = self.base.coords {
    		let orientation = self.layout_orientation();
			let (lp,tp,_,_) = self.base.control_base.layout.padding.into();
	    	let (lm,tm,rm,bm) = self.base.control_base.layout.margin.into();
	    	self.base.widget.get_parent().unwrap().downcast::<Fixed>().unwrap().move_(&self.base.widget, x as i32 + lm, y as i32 + tm);
			self.base.widget.set_size_request(self.base.measured_size.0 as i32 - lm - rm, self.base.measured_size.1 as i32 - rm - bm);
	        let mut x = x + lp + lm;
	        let mut y = y + tp + tm;
	        for ref mut child in self.children.as_mut_slice() {
	            child.draw(Some((x, y)));
	            let (xx, yy) = child.size();
	            match orientation {
	                layout::Orientation::Horizontal => x += xx as i32,
	                layout::Orientation::Vertical => y += yy as i32,
	            }
	        }
		}
    	if let types::Visibility::Visible = self.base.control_base.member_base.visibility {
			self.base.widget.show();
		}
    }
    fn measure(&mut self, parent_width: u16, parent_height: u16) -> (u16, u16, bool) {
    	use std::cmp::max;
    	
    	let orientation = self.layout_orientation();
    	let old_size = self.base.measured_size;
    	let (lp,tp,rp,bp) = self.base.control_base.layout.padding.into();
    	let (lm,tm,rm,bm) = self.base.control_base.layout.margin.into();
    	self.base.measured_size = match self.visibility() {
        	types::Visibility::Gone => (0,0),
        	_ => {
        		let mut measured = false;
        		let w = match self.layout_width() {
        			layout::Size::Exact(w) => w,
        			layout::Size::MatchParent => parent_width,
        			layout::Size::WrapContent => {
	        			let mut w = 0;
		                for ref mut child in self.children.as_mut_slice() {
		                    let (cw, _, _) = child.measure(
		                    	max(0, parent_width as i32 - lp - rp - lm - rm) as u16, 
		                    	max(0, parent_height as i32 - tp - bp - tm - bm) as u16
		                    );
		                    match orientation {
		                    	layout::Orientation::Horizontal => {
			                    	w += cw;
			                    },
		                    	layout::Orientation::Vertical => {
			                    	w = max(w, cw);
			                    },
		                    }
		                }
	        			measured = true;
	        			max(0, w as i32 + lm + rm + lp + rp) as u16
        			}
        		};
        		let h = match self.layout_height() {
        			layout::Size::Exact(h) => h,
        			layout::Size::MatchParent => parent_height,
        			layout::Size::WrapContent => {
	        			let mut h = 0;
		                for ref mut child in self.children.as_mut_slice() {
		                    let ch = if measured {
		                    	child.size().1
		                    } else {
		                    	let (_, ch, _) = child.measure(
			                    	max(0, parent_width as i32 - lp - rp - lm - rm) as u16, 
			                    	max(0, parent_height as i32 - tp - bp - tm - bm) as u16
			                    );
		                    	ch
		                    };
		                    match orientation {
		                    	layout::Orientation::Horizontal => {
			                    	h = max(h, ch);
			                    },
		                    	layout::Orientation::Vertical => {
			                    	h += ch;
			                    },
		                    }
		                }
	        			max(0, h as i32 + tm + bm + tp + bp) as u16
        			}
        		};
        		(w, h)
        	}
        };
    	(self.base.measured_size.0, self.base.measured_size.1, self.base.measured_size != old_size)
    }
}

impl UiHasLayout for LinearLayout {
	fn layout_width(&self) -> layout::Size {
    	self.base.control_base.layout.width
    }
	fn layout_height(&self) -> layout::Size {
		self.base.control_base.layout.height
	}
	fn layout_gravity(&self) -> layout::Gravity {
		self.base.control_base.layout.gravity
	}
	fn layout_alignment(&self) -> layout::Alignment {
		self.base.control_base.layout.alignment
	}
	fn layout_padding(&self) -> layout::BoundarySize {
		self.base.control_base.layout.padding
	}
	fn layout_margin(&self) -> layout::BoundarySize {
		self.base.control_base.layout.margin
	}
	
	fn set_layout_padding(&mut self, padding: layout::BoundarySizeArgs) {
		self.base.control_base.layout.padding = padding.into();
		self.base.invalidate();
	}
	fn set_layout_margin(&mut self, margin: layout::BoundarySizeArgs) {
		self.base.control_base.layout.margin = margin.into();
		self.base.invalidate();
	} 
	fn set_layout_width(&mut self, width: layout::Size) {
		self.base.control_base.layout.width = width;
		self.base.invalidate();
	}
	fn set_layout_height(&mut self, height: layout::Size) {
		self.base.control_base.layout.height = height;
		self.base.invalidate();
	}
	fn set_layout_gravity(&mut self, gravity: layout::Gravity) {
		self.base.control_base.layout.gravity = gravity;
		self.base.invalidate();
	}
	fn set_layout_alignment(&mut self, alignment: layout::Alignment) {
		self.base.control_base.layout.alignment = alignment;
		self.base.invalidate();
	}   
	fn as_member(&self) -> &UiMember {
		self
	}
	fn as_member_mut(&mut self) -> &mut UiMember {
		self
	}
}

impl UiControl for LinearLayout {
	fn is_container_mut(&mut self) -> Option<&mut UiContainer> {
		Some(self)
	}
    fn is_container(&self) -> Option<&UiContainer> {
    	Some(self)
    }
    
    fn parent(&self) -> Option<&types::UiMemberBase> {
        self.base.parent()
    }
    fn parent_mut(&mut self) -> Option<&mut types::UiMemberBase> {
        self.base.parent_mut()
    }
    fn root(&self) -> Option<&types::UiMemberBase> {
        self.base.root()
    }
    fn root_mut(&mut self) -> Option<&mut types::UiMemberBase> {
        self.base.root_mut()
    }
    fn on_added_to_container(&mut self, parent: &UiContainer, x: i32, y: i32) {
    	use plygui_api::development::UiDrawable;
    	
        let (pw, ph) = parent.draw_area_size();
        self.measure(pw, ph);
        self.draw(Some((x, y)));
        
        let selfptr = self as *mut _ as *mut c_void;
        let orientation = self.layout_orientation();
        let (lp,tp,_,_) = self.base.control_base.layout.padding.into();
    	let (lm,tm,_,_) = self.base.control_base.layout.margin.into();
        let mut x = x + lp + lm;
        let mut y = y + tp + tm;
        for ref mut child in self.children.as_mut_slice() {
            let self2: &mut LinearLayout = unsafe { mem::transmute(selfptr) };
            child.on_added_to_container(self2, x, y);
            let (xx, yy) = child.size();
            match orientation {
                layout::Orientation::Horizontal => x += xx as i32,
                layout::Orientation::Vertical => y += yy as i32,
            }
        }
    }
    fn on_removed_from_container(&mut self, _: &UiContainer) {
	    let selfptr = self as *mut _ as *mut c_void;
        for mut child in self.children.drain(..) {
            let self2: &mut LinearLayout = unsafe { mem::transmute(selfptr) };
            child.on_removed_from_container(self2);
        }
    }	

    #[cfg(feature = "markup")]
    fn fill_from_markup(&mut self, markup: &plygui_api::markup::Markup, registry: &mut plygui_api::markup::MarkupRegistry) {
    	use plygui_api::markup::MEMBER_TYPE_LINEAR_LAYOUT;
    	
    	fill_from_markup_base!(self, markup, registry, LinearLayout, [MEMBER_ID_LAYOUT_LINEAR, MEMBER_TYPE_LINEAR_LAYOUT]);
		fill_from_markup_children!(self, markup, registry);		
    }

    fn as_has_layout(&self) -> &UiHasLayout {
    	self
    }
    fn as_has_layout_mut(&mut self) -> &mut UiHasLayout {
    	self
    }
}

impl UiHasOrientation for LinearLayout {
	fn layout_orientation(&self) -> layout::Orientation {
    	self.orientation
    }
    fn set_layout_orientation(&mut self, orientation: layout::Orientation) {
    	self.orientation = orientation;
		self.base.invalidate();
    }
}

impl UiContainer for LinearLayout {
    fn find_control_by_id_mut(&mut self, id_: ids::Id) -> Option<&mut UiControl> {
        if self.as_base().id() == id_ {
            return Some(self);
        }
        for child in self.children.as_mut_slice() {
            if child.as_base().id() == id_ {
                return Some(child.as_mut());
            } else if let Some(c) = child.is_container_mut() {
                let ret = c.find_control_by_id_mut(id_);
                if ret.is_none() {
                    continue;
                }
                return ret;
            }
        }
        None
    }
    fn find_control_by_id(&self, id_: ids::Id) -> Option<&UiControl> {
        if self.as_base().id() == id_ {
            return Some(self);
        }
        for child in self.children.as_slice() {
            if child.as_base().id() == id_ {
                return Some(child.as_ref());
            } else if let Some(c) = child.is_container() {
                let ret = c.find_control_by_id(id_);
                if ret.is_none() {
                    continue;
                }
                return ret;
            }
        }
        None
    }
    fn is_multi_mut(&mut self) -> Option<&mut UiMultiContainer> {
        Some(self)
    }
    fn is_multi(&self) -> Option<&UiMultiContainer> {
        Some(self)
    }
    fn as_member(&self) -> &UiMember {
    	self
    }
	fn as_member_mut(&mut self) -> &mut UiMember {
		self
	}
}

impl UiMultiContainer for LinearLayout {
	fn len(&self) -> usize {
        self.children.len()
    }
    fn set_child_to(&mut self, index: usize, child: Box<UiControl>) -> Option<Box<UiControl>> {
        self.children.insert(index, child);
        unsafe {
    		let base = common::cast_uicommon_to_gtkcommon_mut(mem::transmute(self.children.get_mut(index).unwrap().as_base_mut()));
    		self.base.widget.clone().downcast::<Fixed>().unwrap().add(&base.widget);
    	}
        if (index + 1) >= self.children.len() {
            return None;
        }
        Some(self.children.remove(index + 1))
    }
    fn remove_child_from(&mut self, index: usize) -> Option<Box<UiControl>> {
        if index < self.children.len() {
        	let mut item = self.children.remove(index);
        	unsafe {
	        	let base = common::cast_uicommon_to_gtkcommon_mut(mem::transmute(item.as_base_mut()));					
	        	self.base.widget.clone().downcast::<Fixed>().unwrap().remove(&base.widget);
	        }
	        Some(item)
        } else {
            None
        }
    }
    fn child_at(&self, index: usize) -> Option<&Box<UiControl>> {
        self.children.get(index)
    }
    fn child_at_mut(&mut self, index: usize) -> Option<&mut Box<UiControl>> {
        self.children.get_mut(index)
    }
    fn as_container(&self) -> &UiContainer {
    	self
    }
	fn as_container_mut(&mut self) -> &mut UiContainer {
		self
	}
}

impl UiLinearLayout for LinearLayout {
    fn as_control(&self) -> &UiControl {
    	self
    }
    fn as_control_mut(&mut self) -> &mut UiControl {
    	self
    }
    fn as_multi_container(&self) -> &UiMultiContainer {
    	self
    }
    fn as_multi_container_mut(&mut self) -> &mut UiMultiContainer {
    	self
    }
    fn as_has_orientation(&self) -> &UiHasOrientation {
    	self
    }
    fn as_has_orientation_mut(&mut self) -> &mut UiHasOrientation {
    	self
    }
}

#[allow(dead_code)]
pub(crate) fn spawn() -> Box<UiControl> {
	LinearLayout::new(layout::Orientation::Vertical)
}

impl_invalidate!(LinearLayout);
impl_is_control!(LinearLayout);
impl_size!(LinearLayout);
impl_member_id!(MEMBER_ID_LAYOUT_LINEAR);

fn on_resize_move(this: &Widget, allo: &Rectangle) {
	let mut ll = this.clone().upcast::<Widget>();
	let ll = common::cast_gtk_widget_to_uimember_mut::<LinearLayout>(&mut ll).unwrap();
	if ll.base.measured_size.0 as i32 != allo.width || ll.base.measured_size.1 as i32 != allo.height {
		use std::cmp::max;
		
		ll.base.measured_size = (max(0, allo.width) as u16, max(0, allo.height) as u16);
		if let Some(ref mut cb) = ll.base.h_resize {
            let mut w2 = this.clone().upcast::<Widget>();
			let mut w2 = common::cast_gtk_widget_to_uimember_mut::<LinearLayout>(&mut w2).unwrap();
			(cb.as_mut())(w2, ll.base.measured_size.0 as u16, ll.base.measured_size.1 as u16);
        }
	}
}

