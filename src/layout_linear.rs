use super::*;

use plygui_api::{layout, ids, types, controls, utils};
use plygui_api::development::*;

use gtk::{Cast, Widget, WidgetExt, Box as GtkBox, OrientableExt, ContainerExt};

use std::mem;

const DEFAULT_PADDING: i32 = 0;

pub type LinearLayout = Member<Control<MultiContainer<GtkLinearLayout>>>;

#[repr(C)]
pub struct GtkLinearLayout {
    base: common::GtkControlBase<LinearLayout>,
    gravity_horizontal: layout::Gravity,
    gravity_vertical: layout::Gravity,
    children: Vec<Box<controls::Control>>,
}

impl LinearLayoutInner for GtkLinearLayout {
	fn with_orientation(orientation: layout::Orientation) -> Box<LinearLayout> {
		use plygui_api::controls::HasLayout;
		
		let mut ll = Box::new(Member::with_inner(Control::with_inner(MultiContainer::with_inner(GtkLinearLayout {
                     base: common::GtkControlBase::with_gtk_widget(GtkBox::new(common::orientation_to_gtk(orientation), 0).upcast::<Widget>()),
                     gravity_horizontal: layout::Gravity::default(),
					    gravity_vertical: layout::Gravity::default(),
					    children: Vec::new(),
                 }, ()), ()), MemberFunctions::new(_as_any, _as_any_mut, _as_member, _as_member_mut)));
        {
        	let ptr = ll.as_ref() as *const _ as *mut std::os::raw::c_void;
        	ll.as_inner_mut().as_inner_mut().as_inner_mut().base.set_pointer(ptr);
        }
        ll.set_layout_padding(layout::BoundarySize::AllTheSame(DEFAULT_PADDING).into());
        ll.as_inner_mut().as_inner_mut().as_inner_mut().base.widget.connect_size_allocate(on_size_allocate);
        ll
	}
}

impl MemberInner for GtkLinearLayout {
	type Id = common::GtkWidget;
	
    fn size(&self) -> (u16, u16) {
    	self.base.measured_size
    }
    
    fn on_set_visibility(&mut self, _: &mut MemberBase) {
    	self.base.invalidate()
    }
    
    unsafe fn native_id(&self) -> Self::Id {
    	self.base.widget.clone()
    }
}

impl Drawable for GtkLinearLayout {
	fn draw(&mut self, base: &mut MemberControlBase, coords: Option<(i32, i32)>) {
		self.base.draw(base, coords);
    	if let Some((x, y)) = self.base.coords {
    		let orientation = self.layout_orientation();
			let (lp,tp,_,_) = base.control.layout.padding.into();
	    	let (lm,tm,_,_) = base.control.layout.margin.into();
	    	
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
    	
	}
    fn measure(&mut self, base: &mut MemberControlBase, parent_width: u16, parent_height: u16) -> (u16, u16, bool) {
    	use std::cmp::max;
    	
    	let orientation = self.layout_orientation();
    	let old_size = self.base.measured_size;
    	let (lp,tp,rp,bp) = base.control.layout.padding.into();
    	let (lm,tm,rm,bm) = base.control.layout.margin.into();
    	self.base.measured_size = match base.member.visibility {
        	types::Visibility::Gone => (0,0),
        	_ => {
        		let mut measured = false;
        		let w = match base.control.layout.width {
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
        		let h = match base.control.layout.height {
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
    	(
            self.base.measured_size.0,
            self.base.measured_size.1,
            self.base.measured_size != old_size,
        )
    }
    fn invalidate(&mut self, _: &mut MemberControlBase) {
    	self.base.invalidate()
    }
}

impl HasLayoutInner for GtkLinearLayout {
	fn on_layout_changed(&mut self, _: &mut MemberBase) {
		self.base.invalidate()
	}
}

impl ControlInner for GtkLinearLayout {
	fn on_added_to_container(&mut self, base: &mut MemberControlBase, parent: &controls::Container, x: i32, y: i32) {
		let (pw, ph) = parent.draw_area_size();
        self.measure(base, pw, ph);
        self.draw(base, Some((x, y)));
        
        let orientation = self.layout_orientation();
        let self2 = common::cast_gtk_widget_to_member_mut::<LinearLayout>(&mut self.base.widget).unwrap();
        let (lp,tp,_,_) = base.control.layout.padding.into();
    	let (lm,tm,_,_) = base.control.layout.margin.into();
        let mut x = x + lp + lm;
        let mut y = y + tp + tm;
        for ref mut child in self.children.as_mut_slice() {
            child.on_added_to_container(self2, x, y);
            let (xx, yy) = child.size();
            match orientation {
                layout::Orientation::Horizontal => x += xx as i32,
                layout::Orientation::Vertical => y += yy as i32,
            }
        }
	}
    fn on_removed_from_container(&mut self, _: &mut MemberControlBase, _: &controls::Container) {
    	let self2 = common::cast_gtk_widget_to_member_mut::<LinearLayout>(&mut self.base.widget).unwrap();
        for mut child in self.children.drain(..) {
            child.on_removed_from_container(self2);
        }
    }
    
    fn parent(&self) -> Option<&controls::Member> {
    	self.base.parent().map(|m| m.as_member())
    }
    fn parent_mut(&mut self) -> Option<&mut controls::Member> {
    	self.base.parent_mut().map(|m| m.as_member_mut())
    }
    fn root(&self) -> Option<&controls::Member> {
    	self.base.root().map(|m| m.as_member())
    }
    fn root_mut(&mut self) -> Option<&mut controls::Member> {
    	self.base.root_mut().map(|m| m.as_member_mut())
    }
    
    #[cfg(feature = "markup")]
    fn fill_from_markup(&mut self, base: &mut MemberControlBase, mberarkup: &super::markup::Markup, registry: &mut super::markup::MarkupRegistry) {
    	use plygui_api::markup::MEMBER_TYPE_LINEAR_LAYOUT;
    	
    	fill_from_markup_base!(self, markup, registry, LinearLayout, [MEMBER_TYPE_LINEAR_LAYOUT]);
		fill_from_markup_children!(self, markup, registry);	
    }
}

impl HasOrientationInner for GtkLinearLayout {
	fn layout_orientation(&self) -> layout::Orientation {
	    let widget: Widget = self.base.widget.clone().into();
	    let gtk_self = widget.downcast::<GtkBox>().unwrap();
    	common::gtk_to_orientation(gtk_self.get_orientation())
    }
    fn set_layout_orientation(&mut self, _: &mut MemberBase, orientation: layout::Orientation) {
    	let widget: Widget = self.base.widget.clone().into();
	    let gtk_self = widget.downcast::<GtkBox>().unwrap();
    	gtk_self.set_orientation(common::orientation_to_gtk(orientation));
		self.base.invalidate();
    }
}

impl ContainerInner for GtkLinearLayout {
	fn find_control_by_id_mut(&mut self, id: ids::Id) -> Option<&mut controls::Control> {
		for child in self.children.as_mut_slice() {
            if child.as_member().id() == id {
                return Some(child.as_mut());
            } else if let Some(c) = child.is_container_mut() {
                let ret = c.find_control_by_id_mut(id);
                if ret.is_none() {
                    continue;
                }
                return ret;
            }
        }
        None
	}
    fn find_control_by_id(&self, id: ids::Id) -> Option<&controls::Control> {
    	for child in self.children.as_slice() {
            if child.as_member().id() == id {
                return Some(child.as_ref());
            } else if let Some(c) = child.is_container() {
                let ret = c.find_control_by_id(id);
                if ret.is_none() {
                    continue;
                }
                return ret;
            }
        }
        None
    }
    fn gravity(&self) -> (layout::Gravity, layout::Gravity) {
    	(self.gravity_horizontal, self.gravity_vertical)
    }
    fn set_gravity(&mut self, base: &mut MemberBase, w: layout::Gravity, h: layout::Gravity) {
    	if self.gravity_horizontal != w || self.gravity_vertical != h {
    		self.gravity_horizontal = w;
    		self.gravity_vertical = h;
    		self.invalidate(unsafe { mem::transmute(base) });
    	}
    }
}

impl MultiContainerInner for GtkLinearLayout {
	fn len(&self) -> usize {
		self.children.len()
	}
    fn set_child_to(&mut self, base: &mut MemberBase, index: usize, child: Box<controls::Control>) -> Option<Box<controls::Control>> {
    	let self2 = unsafe { utils::base_to_impl_mut::<LinearLayout>(base) };
        
        self.children.insert(index, child);
        let old = if (index + 1) < self.children.len() {
            let mut old = self.children.remove(index + 1);
            old.on_removed_from_container(self2);
            Some(old)
        } else {
            None
        };
        
        let widget = common::cast_control_to_gtkwidget(self.children.get_mut(index).unwrap().as_mut());
    	let self_widget: gtk::Widget = self.base.widget.clone().into();
    	self_widget.downcast::<GtkBox>().unwrap().add::<Widget>(&widget.into());
    	self.children.get_mut(index).unwrap().on_added_to_container(self2, 0, 0);
        old
    }
    fn remove_child_from(&mut self, _: &mut MemberBase, index: usize) -> Option<Box<controls::Control>> {
    	if index < self.children.len() {
        	let item = self.children.remove(index);
        	let widget = common::cast_control_to_gtkwidget(item.as_ref());					
	        let self_widget: gtk::Widget = self.base.widget.clone().into();
    		self_widget.downcast::<GtkBox>().unwrap().remove::<Widget>(&widget.into());
	        
	        Some(item)
        } else {
            None
        }
    }
    fn child_at(&self, index: usize) -> Option<&controls::Control> {
    	self.children.get(index).map(|m| m.as_ref())
    }
    fn child_at_mut(&mut self, index: usize) -> Option<&mut controls::Control> {
    	//self.children.get_mut(index).map(|c| c.as_mut()) //the anonymous lifetime #1 does not necessarily outlive the static lifetime
        if let Some(c) = self.children.get_mut(index) {
        	Some(c.as_mut())
        } else {
        	None
        }
    }
}

#[allow(dead_code)]
pub(crate) fn spawn() -> Box<controls::Control> {
	LinearLayout::with_orientation(layout::Orientation::Vertical).into_control()
}

fn on_size_allocate(this: &::gtk::Widget, _allo: &::gtk::Rectangle) {
	let mut ll = this.clone().upcast::<Widget>();
	let ll = common::cast_gtk_widget_to_member_mut::<LinearLayout>(&mut ll).unwrap();
	
	let measured_size = ll.as_inner_mut().as_inner_mut().as_inner_mut().base.measured_size;
	if let Some(ref mut cb) = ll.base_mut().handler_resize {
        let mut w2 = this.clone().upcast::<Widget>();
		let mut w2 = common::cast_gtk_widget_to_member_mut::<LinearLayout>(&mut w2).unwrap();
		(cb.as_mut())(w2, measured_size.0 as u16, measured_size.1 as u16);
    }
}

impl_all_defaults!(LinearLayout);
