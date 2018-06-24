use super::*;

use plygui_api::{layout, ids, types, development, controls};
use plygui_api::development::*;

use gtk::{Cast, Widget, WidgetExt, Fixed, FixedExt, Paned, PanedExt, OrientableExt};

use std::mem;

const DEFAULT_PADDING: i32 = 0;

pub type Splitted = development::Member<development::Control<development::MultiContainer<GtkSplitted>>>;

#[repr(C)]
pub struct GtkSplitted {
    base: common::GtkControlBase<Splitted>,
    gravity_horizontal: layout::Gravity,
    gravity_vertical: layout::Gravity,
    
    splitter: f32,
    first: Box<controls::Control>, 
    second: Box<controls::Control>, 
}

impl GtkSplitted {
    fn update_splitter(&mut self) {
        let self_widget: gtk::Widget = self.base.widget.clone().into();
		let orientation = self.layout_orientation();
		match orientation {
		    layout::Orientation::Horizontal => self_widget.downcast::<Paned>().unwrap().set_position((self.base.measured_size.0 as f32 * self.splitter) as i32),
		    layout::Orientation::Vertical => self_widget.downcast::<Paned>().unwrap().set_position((self.base.measured_size.1 as f32 * self.splitter) as i32),
		}
    }
}

impl development::SplittedInner for GtkSplitted {
	fn with_content(first: Box<controls::Control>, second: Box<controls::Control>, orientation: layout::Orientation) -> Box<controls::Splitted> {
	    use plygui_api::controls::HasLayout;
		
		let mut ll = Box::new(development::Member::with_inner(development::Control::with_inner(development::MultiContainer::with_inner(GtkSplitted {
                     base: common::GtkControlBase::with_gtk_widget(Paned::new(common::orientation_to_gtk(orientation)).upcast::<Widget>()),
                     gravity_horizontal: layout::Gravity::default(),
					    gravity_vertical: layout::Gravity::default(),
					  first: first,
					  splitter: 0.5,
					  second: second,
                 }, ()), ()), development::MemberFunctions::new(_as_any, _as_any_mut, _as_member, _as_member_mut)));
        {
        	let ptr = ll.as_ref() as *const _ as *mut std::os::raw::c_void;
        	ll.as_inner_mut().as_inner_mut().as_inner_mut().base.set_pointer(ptr);
        }
        {
            use plygui_api::controls::Splitted;
            
            let self_widget: gtk::Widget = ll.as_inner_mut().as_inner_mut().as_inner_mut().base.widget.clone().into();
        	let gtk_self = self_widget.downcast::<Paned>().unwrap();
        	let paned = gtk_self.downcast::<Paned>().unwrap();
        	paned.add1(common::cast_control_to_gtkwidget(ll.first()).as_ref());
            paned.add2(common::cast_control_to_gtkwidget(ll.second()).as_ref());
            paned.connect_property_position_notify(on_property_position_notify);
        }
        ll.set_layout_padding(layout::BoundarySize::AllTheSame(DEFAULT_PADDING).into());
        ll.as_inner_mut().as_inner_mut().as_inner_mut().base.widget.connect_size_allocate(on_size_allocate);
        ll
	}
	fn set_splitter(&mut self, _: &mut MemberControlBase, pos: f32) {
	    let pos = pos % 1.0;
	    self.splitter = pos;
	    self.update_splitter();
	}
	fn splitter(&self) -> f32 {
	    self.splitter
	}
	
	fn first(&self) -> &controls::Control { self.first.as_ref() }
	fn second(&self) -> &controls::Control { self.second.as_ref() }
	fn first_mut(&mut self) -> &mut controls::Control { self.first.as_mut() }
	fn second_mut(&mut self) -> &mut controls::Control { self.second.as_mut() }
}

impl development::MemberInner for GtkSplitted {
	type Id = common::GtkWidget;
	
    fn size(&self) -> (u16, u16) {
    	self.base.measured_size
    }
    
    fn on_set_visibility(&mut self, _: &mut development::MemberBase) {
    	self.base.invalidate()
    }
    
    unsafe fn native_id(&self) -> Self::Id {
    	self.base.widget.clone()
    }
}

impl development::Drawable for GtkSplitted {
	fn draw(&mut self, base: &mut development::MemberControlBase, coords: Option<(i32, i32)>) {
		if coords.is_some() {
    		self.base.coords = coords;
    	}
    	if let Some((x, y)) = self.base.coords {
    	    self.update_splitter();
    		let orientation = self.layout_orientation();
			let (lp,tp,_,_) = base.control.layout.padding.into();
	    	let (lm,tm,rm,bm) = base.control.layout.margin.into();
	    	self.base.widget.get_parent().unwrap().downcast::<Fixed>().unwrap().move_::<Widget>(&(self.base.widget.clone().into()), x as i32 + lm, y as i32 + tm);
			self.base.widget.set_size_request(self.base.measured_size.0 as i32 - lm - rm, self.base.measured_size.1 as i32 - rm - bm);
	        let mut x = x + lp + lm;
	        let mut y = y + tp + tm;
	        for ref mut child in [self.first.as_mut(), self.second.as_mut()].iter_mut() {
	            child.draw(Some((x, y)));
	            let (xx, yy) = child.size();
	            match orientation {
	                layout::Orientation::Horizontal => x += xx as i32,
	                layout::Orientation::Vertical => y += yy as i32,
	            }
	        }
		}
    	if let types::Visibility::Visible = base.member.visibility {
			self.base.widget.show();
		} else {
			self.base.widget.hide();
		}
    	self.base.dirty = false;
	}
    fn measure(&mut self, base: &mut development::MemberControlBase, parent_width: u16, parent_height: u16) -> (u16, u16, bool) {
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
		                for ref mut child in [self.first.as_mut(), self.second.as_mut()].iter_mut() {
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
		                for ref mut child in [self.first.as_mut(), self.second.as_mut()].iter_mut() {
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
    	self.base.dirty = self.base.measured_size != old_size;
        (
            self.base.measured_size.0,
            self.base.measured_size.1,
            self.base.dirty,
        )
    }
    fn invalidate(&mut self, _: &mut development::MemberControlBase) {
    	self.base.invalidate()
    }
}

impl development::HasLayoutInner for GtkSplitted {
	fn on_layout_changed(&mut self, _: &mut development::MemberBase) {
		self.base.invalidate()
	}
}

impl development::ControlInner for GtkSplitted {
	fn on_added_to_container(&mut self, base: &mut development::MemberControlBase, parent: &controls::Container, x: i32, y: i32) {
		let (pw, ph) = parent.draw_area_size();
        self.measure(base, pw, ph);
        self.base.dirty = false;
        self.draw(base, Some((x, y)));
        
        let orientation = self.layout_orientation();
        let self2 = common::cast_gtk_widget_to_member_mut::<Splitted>(&mut self.base.widget).unwrap();
        let (lp,tp,_,_) = base.control.layout.padding.into();
    	let (lm,tm,_,_) = base.control.layout.margin.into();
        let mut x = x + lp + lm;
        let mut y = y + tp + tm;
        for ref mut child in [self.first.as_mut(), self.second.as_mut()].iter_mut() {
            child.on_added_to_container(self2, x, y);
            let (xx, yy) = child.size();
            match orientation {
                layout::Orientation::Horizontal => x += xx as i32,
                layout::Orientation::Vertical => y += yy as i32,
            }
        }
	}
    fn on_removed_from_container(&mut self, _: &mut development::MemberControlBase, _: &controls::Container) {
    	let self2 = common::cast_gtk_widget_to_member_mut::<Splitted>(&mut self.base.widget).unwrap();
        for mut child in [self.first.as_mut(), self.second.as_mut()].iter_mut() {
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
    fn fill_from_markup(&mut self, base: &mut development::MemberControlBase, mberarkup: &super::markup::Markup, registry: &mut super::markup::MarkupRegistry) {
    	use plygui_api::markup::MEMBER_TYPE_LINEAR_LAYOUT;
    	
    	fill_from_markup_base!(self, markup, registry, Splitted, [MEMBER_TYPE_LINEAR_LAYOUT]);
		fill_from_markup_children!(self, markup, registry);	
    }
}

impl development::HasOrientationInner for GtkSplitted {
	fn layout_orientation(&self) -> layout::Orientation {
	    let widget: Widget = self.base.widget.clone().into();
	    let gtk_self = widget.downcast::<Paned>().unwrap();
    	common::gtk_to_orientation(gtk_self.get_orientation())
    }
    fn set_layout_orientation(&mut self, _: &mut development::MemberBase, orientation: layout::Orientation) {
    	let widget: Widget = self.base.widget.clone().into();
	    let gtk_self = widget.downcast::<Paned>().unwrap();
    	gtk_self.set_orientation(common::orientation_to_gtk(orientation));
		self.base.invalidate();
    }
}

impl development::ContainerInner for GtkSplitted {
	fn find_control_by_id_mut(&mut self, id: ids::Id) -> Option<&mut controls::Control> {
		use plygui_api::development::SplittedInner;
		
		if self.first().as_member().id() == id {
			return Some(self.first_mut());
		}
		if self.second().as_member().id() == id {
			return Some(self.second_mut());
		}
		
		let self2: &mut GtkSplitted = unsafe { mem::transmute(self as *mut GtkSplitted) }; // bck is stupid
		if let Some(c) = self.first_mut().is_container_mut() {
            let ret = c.find_control_by_id_mut(id);
            if ret.is_some() {
                return ret;
            }
        }
		if let Some(c) = self2.second_mut().is_container_mut() {
            let ret = c.find_control_by_id_mut(id);
            if ret.is_some() {
                return ret;
            }
        }
		
        None
	}
    fn find_control_by_id(&self, id: ids::Id) -> Option<&controls::Control> {
    	use plygui_api::development::SplittedInner;
    	
        if self.first().as_member().id() == id {
			return Some(self.first());
		}
		if self.second().as_member().id() == id {
			return Some(self.second());
		}
		
		if let Some(c) = self.first().is_container() {
            let ret = c.find_control_by_id(id);
            if ret.is_some() {
                return ret;
            }
        }
		if let Some(c) = self.second().is_container() {
            let ret = c.find_control_by_id(id);
            if ret.is_some() {
                return ret;
            }
        }
		
        None
    }
    fn gravity(&self) -> (layout::Gravity, layout::Gravity) {
    	(self.gravity_horizontal, self.gravity_vertical)
    }
    fn set_gravity(&mut self, base: &mut development::MemberBase, w: layout::Gravity, h: layout::Gravity) {
    	if self.gravity_horizontal != w || self.gravity_vertical != h {
    		self.gravity_horizontal = w;
    		self.gravity_vertical = h;
    		self.invalidate(unsafe { mem::transmute(base) });
    	}
    }
}

impl development::MultiContainerInner for GtkSplitted {
	fn len(&self) -> usize {
		2
	}
    fn set_child_to(&mut self, _: &mut development::MemberBase, index: usize, mut child: Box<controls::Control>) -> Option<Box<controls::Control>> {
    	use plygui_api::controls::HasLayout;
    	
    	let orientation = self.layout_orientation();
    	let self_widget: gtk::Widget = self.base.widget.clone().into();
    	let gtk_self = self_widget.downcast::<Paned>().unwrap();
		let self2 = common::cast_gtk_widget_to_member_mut::<Splitted>(&mut self.base.widget).unwrap();
		let (lp, tp, _, _) = self2.as_has_layout().layout_padding().into();
        let (lm, tm, _, _) = self2.as_has_layout().layout_margin().into();
		    
	    match index {
	    	0 => {
	    	    self.first.on_removed_from_container(self2);
			    child.on_added_to_container(self2, lp + lm, tp + tm);
	    		mem::swap(&mut self.first, &mut child);
	    		
	    		let widget = common::cast_control_to_gtkwidget(self.first.as_mut());
	    		gtk_self.add1(widget.as_ref());
	    	},
	    	1 => {
	    		let mut x = lp + lm;
			    let mut y = tp + tm;
    
		        let (xx, yy) = self.first.size();
				match orientation {
				    layout::Orientation::Horizontal => { 
				    	x += xx as i32;
				    },
				    layout::Orientation::Vertical => {
				    	y += yy as i32;
					},
				} 
		        
	    		self.second.on_removed_from_container(self2);
	    		child.on_added_to_container(self2, x, y);
	    		mem::swap(&mut self.second, &mut child);
	    		
	    		let widget = common::cast_control_to_gtkwidget(self.first.as_mut());
	    		gtk_self.downcast::<Paned>().unwrap().add2(widget.as_ref());
	    	},
	    	_ => return None,
    	}
    	
    	Some(child)
    }
    fn remove_child_from(&mut self, _: &mut development::MemberBase, _: usize) -> Option<Box<controls::Control>> {
    	None
    }
    fn child_at(&self, index: usize) -> Option<&controls::Control> {
    	use plygui_api::development::SplittedInner;
    	
    	match index {
    		0 => Some(self.first()),
    		1 => Some(self.second()),
    		_ => None
    	}
    }
    fn child_at_mut(&mut self, index: usize) -> Option<&mut controls::Control> {
    	use plygui_api::development::SplittedInner;
    	
    	match index {
    		0 => Some(self.first_mut()),
    		1 => Some(self.second_mut()),
    		_ => None
    	}
    }
}

/*#[allow(dead_code)]
pub(crate) fn spawn() -> Box<controls::Control> {
	Splitted::with_orientation(layout::Orientation::Vertical).into_control()
}*/

fn on_size_allocate(this: &::gtk::Widget, _allo: &::gtk::Rectangle) {
    let mut ll = this.clone().upcast::<Widget>();
	let ll = common::cast_gtk_widget_to_member_mut::<Splitted>(&mut ll).unwrap();
	if ll.as_inner_mut().as_inner_mut().as_inner_mut().base.dirty {
		ll.as_inner_mut().as_inner_mut().as_inner_mut().base.dirty = false;
		let measured_size = ll.as_inner_mut().as_inner_mut().as_inner_mut().base.measured_size;
		if let Some(ref mut cb) = ll.base_mut().handler_resize {
            let mut w2 = this.clone().upcast::<Widget>();
			let mut w2 = common::cast_gtk_widget_to_member_mut::<Splitted>(&mut w2).unwrap();
			(cb.as_mut())(w2, measured_size.0 as u16, measured_size.1 as u16);
        }
	}
}
fn on_property_position_notify(this: &::gtk::Paned) {
    use plygui_api::controls::{HasOrientation, Member};
    
    let position = this.get_position();
    let mut ll = this.clone().upcast::<Widget>();
	let ll = common::cast_gtk_widget_to_member_mut::<Splitted>(&mut ll).unwrap();
	let orientation = ll.layout_orientation();
	let size = ll.size();
	let splitter = position as f32 / match orientation {
    	layout::Orientation::Vertical => size.1 as f32,
    	layout::Orientation::Horizontal => size.0 as f32,
	};
	println!("{} was / {} now", ll.as_inner_mut().as_inner_mut().as_inner_mut().splitter, splitter);
	ll.as_inner_mut().as_inner_mut().as_inner_mut().splitter = splitter;
}

impl_all_defaults!(Splitted);
