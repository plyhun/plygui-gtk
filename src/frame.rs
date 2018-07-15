use super::*;

use gtk::{Cast, Widget, WidgetExt, Frame as GtkFrameSys, FrameExt, Label, LabelExt, CssProvider, CssProviderExt, StyleContextExt, ContainerExt};
use pango::LayoutExt;

use plygui_api::{layout, types, controls, utils, ids};
use plygui_api::development::*;

use std::borrow::Cow;
use std::{mem, cmp};

const DEFAULT_PADDING: i32 = 6;

pub type Frame = Member<Control<SingleContainer<GtkFrame>>>;

#[repr(C)]
pub struct GtkFrame {
    base: common::GtkControlBase<Frame>,

    gravity_horizontal: layout::Gravity,
    gravity_vertical: layout::Gravity,
    child: Option<Box<controls::Control>>,
}

impl FrameInner for GtkFrame {
	fn with_label(label: &str) -> Box<Frame> {
		use plygui_api::controls::HasLayout;
		
		let mut fr = Box::new(Member::with_inner(Control::with_inner(SingleContainer::with_inner(GtkFrame {
                     base: common::GtkControlBase::with_gtk_widget(GtkFrameSys::new(label).upcast::<Widget>()),
                     gravity_horizontal: Default::default(),
                    gravity_vertical: Default::default(),
                    child: None
                 }, ()), ()), MemberFunctions::new(_as_any, _as_any_mut, _as_member, _as_member_mut)));
        {
        	let ptr = fr.as_ref() as *const _ as *mut std::os::raw::c_void;
        	fr.as_inner_mut().as_inner_mut().as_inner_mut().base.set_pointer(ptr);
        }
        fr.set_layout_padding(layout::BoundarySize::AllTheSame(DEFAULT_PADDING).into());
        fr.as_inner_mut().as_inner_mut().as_inner_mut().base.widget.connect_size_allocate(on_size_allocate);
        fr
	}
}

impl GtkFrame {
    fn apply_padding(&mut self, base: &ControlBase) {
	    let (lp,tp,rp,bp) = base.layout.padding.into();
			
		let self_widget: gtk::Widget = self.base.widget.clone().into();	
	    let fr = self_widget.downcast::<GtkFrameSys>().unwrap();   
		let css = CssProvider::new();
		css.load_from_data(format!("GtkFrame {{ padding-left: {}px; padding-top: {}px; padding-right: {}px; padding-bottom: {}px; }}", lp, tp, rp, bp).as_bytes()).unwrap();
		fr.get_style_context().unwrap().add_provider(&css, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);
    }
}

impl SingleContainerInner for GtkFrame {
    fn set_child(&mut self, base: &mut MemberBase, mut child: Option<Box<controls::Control>>) -> Option<Box<controls::Control>> {
        let mut old = self.child.take();
        let frame_sys: gtk::Widget = self.base.widget.clone().into();
		let frame_sys = frame_sys.downcast::<GtkFrameSys>().unwrap();
        if let Some(old) = old.as_mut() {
            let old_sys: common::GtkWidget = unsafe { old.native_id() }.into();
            frame_sys.remove(old_sys.as_ref());
            if self.base.coords.is_some() {
        		let self2 = unsafe { utils::base_to_impl_mut::<Frame>(base) };
                old.on_removed_from_container(self2);
            }
        }
        if let Some(new) = child.as_mut() {
        	let widget = common::cast_control_to_gtkwidget(new.as_ref());
    		frame_sys.add(widget.as_ref());
            if self.base.coords.is_some() {
                let self2 = unsafe { utils::base_to_impl_mut::<Frame>(base) };
                new.on_added_to_container(self2, 0, 0);
            }
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

impl ContainerInner for GtkFrame {
    fn find_control_by_id_mut(&mut self, id: ids::Id) -> Option<&mut controls::Control> {
		if let Some(child) = self.child.as_mut() {
            if let Some(c) = child.is_container_mut() {
                return c.find_control_by_id_mut(id);
            }
        }
        None
	}
    fn find_control_by_id(&self, id: ids::Id) -> Option<&controls::Control> {
    	if let Some(child) = self.child.as_ref() {
            if let Some(c) = child.is_container() {
                return c.find_control_by_id(id);
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

impl HasLabelInner for GtkFrame {
	fn label<'a>(&'a self) -> Cow<'a, str> {
		let self_widget: gtk::Widget = self.base.widget.clone().into();
		Cow::Owned(self_widget.downcast::<GtkFrameSys>().unwrap().get_label().unwrap_or(String::new()))
	}
    fn set_label(&mut self, _: &mut MemberBase, label: &str) {
    	let self_widget: gtk::Widget = self.base.widget.clone().into();
    	self_widget.downcast::<GtkFrameSys>().unwrap().set_label(label)
    }
}

impl HasLayoutInner for GtkFrame {
	fn on_layout_changed(&mut self, base: &mut MemberBase) {
		self.apply_padding(unsafe { &mut utils::member_control_base_mut_unchecked(base).control });
		self.base.invalidate();
	}
}

impl ControlInner for GtkFrame {
	fn on_added_to_container(&mut self, base: &mut MemberControlBase, parent: &controls::Container, x: i32, y: i32) {
		let (pw, ph) = parent.draw_area_size();
        self.measure(base, pw, ph);
        self.draw(base, Some((x, y)));
        if let Some(ref mut child) = self.child {
            let self2 = unsafe { utils::base_to_impl_mut::<Frame>(&mut base.member) };
            child.on_added_to_container(self2, 0, 0);
        }
	}
    fn on_removed_from_container(&mut self, base: &mut MemberControlBase, _: &controls::Container) {
        if let Some(ref mut child) = self.child {
            let self2 = unsafe { utils::base_to_impl_mut::<Frame>(&mut base.member) };
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
    	use plygui_api::markup::MEMBER_TYPE_BUTTON;
		fill_from_markup_base!(
            self,
            base,
            markup,
            registry,
            Frame,
            [MEMBER_TYPE_BUTTON]
        );
        fill_from_markup_label!(self, &mut base.member, markup);
        fill_from_markup_callbacks!(self, markup, registry, [on_click => plygui_api::callbacks::Click]);
    }
}

impl MemberInner for GtkFrame {
	type Id = common::GtkWidget;
	
    fn size(&self) -> (u16, u16) {
    	self.base.measured_size
    }
    
    fn on_set_visibility(&mut self, _: &mut MemberBase) {
    	self.base.invalidate()
    }
    
    unsafe fn native_id(&self) -> Self::Id {
    	self.base.widget.clone().into()
    }
}

impl Drawable for GtkFrame {
	fn draw(&mut self, base: &mut MemberControlBase, coords: Option<(i32, i32)>) {
		self.base.draw(base, coords);
	}
    fn measure(&mut self, base: &mut MemberControlBase, parent_width: u16, parent_height: u16) -> (u16, u16, bool) {
    	let old_size = self.base.measured_size;
    	self.base.measured_size = match base.member.visibility {
            types::Visibility::Gone => (0, 0),
            _ => {
                let (lp,tp,rp,bp) = base.control.layout.padding.into();
		    	let (lm,tm,rm,bm) = base.control.layout.margin.into();
		    	    	
		    	let mut label_size = (-1i32, -1i32);
		    	let mut measured = false;
                let w = match base.control.layout.width {
                    layout::Size::MatchParent => parent_width as i32,
                    layout::Size::Exact(w) => w as i32,
                    layout::Size::WrapContent => {
                        if let Some(ref mut child) =  self.child {
		                    let (cw, _, _) = child.measure(
		                    	cmp::max(0, parent_width as i32 - lp - rp) as u16, 
		                    	cmp::max(0, parent_height as i32 - tp - bp) as u16
		                    );
		                    label_size.0 += cw as i32;
		                    measured = true;
		                }	        			
                        if label_size.0 < 0 {
                        	let self_widget: gtk::Widget = self.base.widget.clone().into();
                            let mut frame_sys = self_widget.downcast::<GtkFrameSys>().unwrap();
                        	let mut label = frame_sys.get_label_widget().unwrap().downcast::<Label>().unwrap();		
                        	label_size = label.get_layout().unwrap().get_pixel_size();			
                        }
                        // why the bloody hell I need these?
                        label_size.0 += 4;
                        label_size.1 += 4;
                        
                        label_size.0 + lp + rp + lm + rm
                    } 
                };
                let h = match base.control.layout.height {
                    layout::Size::MatchParent => parent_height as i32,
                    layout::Size::Exact(h) => h as i32,
                    layout::Size::WrapContent => {
                        if let Some(ref mut child) =  self.child {
                            let ch = if measured {
    	                    	child.size().1
    	                    } else {
    	                    	let (_, ch, _) = child.measure(
    		                    	cmp::max(0, parent_width as i32 - lp - rp) as u16, 
    		                    	cmp::max(0, parent_height as i32 - tp - bp) as u16
    		                    );
    	                    	ch
    	                    };
    	                    label_size.1 += ch as i32;
                        }
                        if label_size.1 < 0 {
                        	let self_widget: gtk::Widget = self.base.widget.clone().into();
                            let mut frame_sys = self_widget.downcast::<GtkFrameSys>().unwrap();
                        	let mut label = frame_sys.get_label_widget().unwrap().downcast::<Label>().unwrap();		
                        	label_size = label.get_layout().unwrap().get_pixel_size();	
                        }
                        // why the bloody hell I need these?
                        label_size.0 += 4;
                        label_size.1 += 4;
                        
                        label_size.1 + tp + bp + tm + bm
                    } 
                };
                (cmp::max(0, w) as u16, cmp::max(0, h) as u16)
            },
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

#[allow(dead_code)]
pub(crate) fn spawn() -> Box<controls::Control> {
	Frame::with_label("").into_control()
}

fn on_size_allocate(this: &::gtk::Widget, _allo: &::gtk::Rectangle) {
	let mut ll = this.clone().upcast::<Widget>();
	let ll = common::cast_gtk_widget_to_member_mut::<Frame>(&mut ll).unwrap();
	
	let measured_size = ll.as_inner().as_inner().as_inner().base.measured_size;
	if let Some(ref mut cb) = ll.base_mut().handler_resize {
        let mut w2 = this.clone().upcast::<Widget>();
		let mut w2 = common::cast_gtk_widget_to_member_mut::<Frame>(&mut w2).unwrap();
		(cb.as_mut())(w2, measured_size.0 as u16, measured_size.1 as u16);
    }
}

impl_all_defaults!(Frame);
