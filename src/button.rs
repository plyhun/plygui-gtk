use super::*;

use gtk::{Cast, Widget, WidgetExt, Button as GtkButtonSys, ButtonExt, Bin, BinExt, Label, LabelExt, Fixed, FixedExt, CssProvider, CssProviderExt, StyleContextExt};
use pango::LayoutExt;

use plygui_api::{layout, types, development, callbacks, controls, utils};
use plygui_api::development::{Drawable, HasInner};

use std::borrow::Cow;
use std::cmp::max;

const DEFAULT_PADDING: i32 = 6;

pub type Button = development::Member<development::Control<GtkButton>>;

#[repr(C)]
pub struct GtkButton {
    base: common::GtkControlBase<Button>,

    h_left_clicked: Option<callbacks::Click>,
    h_right_clicked: Option<callbacks::Click>,
}

impl development::ButtonInner for GtkButton {
	fn with_label(label: &str) -> Box<controls::Button> {
		use plygui_api::controls::HasLayout;
		
		let mut btn = Box::new(development::Member::with_inner(development::Control::with_inner(GtkButton {
                     base: common::GtkControlBase::with_gtk_widget(GtkButtonSys::new_with_label(label).upcast::<Widget>()),
                     h_left_clicked: None,
                     h_right_clicked: None,
                 }, ()), development::MemberFunctions::new(_as_any, _as_any_mut, _as_member, _as_member_mut)));
        {
        	let ptr = btn.as_ref() as *const _ as *mut std::os::raw::c_void;
        	btn.as_inner_mut().as_inner_mut().base.set_pointer(ptr);
        }
        {
        	let self_widget: gtk::Widget = btn.as_inner_mut().as_inner_mut().base.widget.clone().into();
        	let button = self_widget.downcast::<GtkButtonSys>().unwrap();
			button.connect_clicked(on_click);
        }
        btn.set_layout_padding(layout::BoundarySize::AllTheSame(DEFAULT_PADDING).into());
        btn.as_inner_mut().as_inner_mut().base.widget.connect_size_allocate(on_size_allocate);
        btn
	}
}

impl GtkButton {
    fn apply_padding(&mut self, base: &development::ControlBase) {
	    let (lp,tp,rp,bp) = base.layout.padding.into();
			
		let self_widget: gtk::Widget = self.base.widget.clone().into();	
	    let btn = self_widget.downcast::<GtkButtonSys>().unwrap();   
		let css = CssProvider::new();
		css.load_from_data(format!("GtkButton {{ padding-left: {}px; padding-top: {}px; padding-right: {}px; padding-bottom: {}px; }}", lp, tp, rp, bp).as_bytes()).unwrap();
		btn.get_style_context().unwrap().add_provider(&css, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);
    }
}

impl development::HasLabelInner for GtkButton {
	fn label<'a>(&'a self) -> Cow<'a, str> {
		let self_widget: gtk::Widget = self.base.widget.clone().into();
		Cow::Owned(self_widget.downcast::<GtkButtonSys>().unwrap().get_label().unwrap_or(String::new()))
	}
    fn set_label(&mut self, _: &mut development::MemberBase, label: &str) {
    	let self_widget: gtk::Widget = self.base.widget.clone().into();
    	self_widget.downcast::<GtkButtonSys>().unwrap().set_label(label)
    }
}

impl development::ClickableInner for GtkButton {
	fn on_click(&mut self, cb: Option<callbacks::Click>) {
		self.h_left_clicked = cb;
    } 
}

impl development::HasLayoutInner for GtkButton {
	fn on_layout_changed(&mut self, base: &mut development::MemberBase) {
		self.apply_padding(unsafe { &mut utils::member_control_base_mut_unchecked(base).control });
		self.base.invalidate();
	}
}

impl development::ControlInner for GtkButton {
	fn on_added_to_container(&mut self, base: &mut development::MemberControlBase, parent: &controls::Container, x: i32, y: i32) {
		let (pw, ph) = parent.draw_area_size();
        self.measure(base, pw, ph);
        self.base.dirty = false;
        self.draw(base, Some((x, y)));
	}
    fn on_removed_from_container(&mut self, _: &mut development::MemberControlBase, _: &controls::Container) {}
    
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
    	use plygui_api::markup::MEMBER_TYPE_BUTTON;
		fill_from_markup_base!(
            self,
            base,
            markup,
            registry,
            Button,
            [MEMBER_TYPE_BUTTON]
        );
        fill_from_markup_label!(self, &mut base.member, markup);
        fill_from_markup_callbacks!(self, markup, registry, [on_click => plygui_api::callbacks::Click]);
    }
}

impl development::MemberInner for GtkButton {
	type Id = common::GtkWidget;
	
    fn size(&self) -> (u16, u16) {
    	self.base.measured_size
    }
    
    fn on_set_visibility(&mut self, _: &mut development::MemberBase) {
    	self.base.invalidate()
    }
    
    unsafe fn native_id(&self) -> Self::Id {
    	self.base.widget.clone().into()
    }
}

impl development::Drawable for GtkButton {
	fn draw(&mut self, base: &mut development::MemberControlBase, coords: Option<(i32, i32)>) {
		if coords.is_some() {
    		self.base.coords = coords;
    	}
    	if let Some(coords) = self.base.coords {
			let (lm,tm,rm,bm) = base.control.layout.margin.into();
	        self.base.widget.set_size_request(self.base.measured_size.0 as i32 - lm - rm, self.base.measured_size.1 as i32 - rm - bm);
			if let types::Visibility::Visible = base.member.visibility {
				self.base.widget.show();
			} else {
				self.base.widget.hide();
			}
		}
        self.base.dirty = false;
	}
    fn measure(&mut self, base: &mut development::MemberControlBase, parent_width: u16, parent_height: u16) -> (u16, u16, bool) {
    	let old_size = self.base.measured_size;
    	println!("parent {} / {}", parent_width, parent_height);
    	self.base.measured_size = match base.member.visibility {
            types::Visibility::Gone => (0, 0),
            _ => {
                let (lp,tp,rp,bp) = base.control.layout.padding.into();
		    	let (lm,tm,rm,bm) = base.control.layout.margin.into();
		    	    	
		    	let mut label_size = (-1i32, -1i32);
                let w = match base.control.layout.width {
                    layout::Size::MatchParent => parent_width as i32,
                    layout::Size::Exact(w) => w as i32,
                    layout::Size::WrapContent => {
                        if label_size.0 < 0 {
                        	let self_widget: gtk::Widget = self.base.widget.clone().into();
                        	let mut bin = self_widget.downcast::<Bin>().unwrap();
                        	let mut label = bin.get_child().unwrap().downcast::<Label>().unwrap();		
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
                        if label_size.1 < 0 {
                        	let self_widget: gtk::Widget = self.base.widget.clone().into();
                            let mut bin = self_widget.downcast::<Bin>().unwrap();
                        	let mut label = bin.get_child().unwrap().downcast::<Label>().unwrap();		
                        	label_size = label.get_layout().unwrap().get_pixel_size();	
                        }
                        // why the bloody hell I need these?
                        label_size.0 += 4;
                        label_size.1 += 4;
                        
                        label_size.1 + tp + bp + tm + bm
                    } 
                };
                (max(0, w) as u16, max(0, h) as u16)
            },
        };
    	println!("{:?}", self.base.measured_size);
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

#[allow(dead_code)]
pub(crate) fn spawn() -> Box<controls::Control> {
	Button::with_label("").into_control()
}

fn on_size_allocate(this: &::gtk::Widget, _allo: &::gtk::Rectangle) {
	let mut ll = this.clone().upcast::<Widget>();
	let ll = common::cast_gtk_widget_to_member_mut::<Button>(&mut ll).unwrap();
	
	if ll.as_inner().as_inner().base.dirty {
		ll.as_inner_mut().as_inner_mut().base.dirty = false;
		let measured_size = ll.as_inner().as_inner().base.measured_size;
		if let Some(ref mut cb) = ll.base_mut().handler_resize {
            let mut w2 = this.clone().upcast::<Widget>();
			let mut w2 = common::cast_gtk_widget_to_member_mut::<Button>(&mut w2).unwrap();
			(cb.as_mut())(w2, measured_size.0 as u16, measured_size.1 as u16);
        }
	}
}

fn on_click(this: &GtkButtonSys) {
	let mut b = this.clone().upcast::<Widget>();
	let b = common::cast_gtk_widget_to_member_mut::<Button>(&mut b).unwrap();
	if let Some(ref mut cb) = b.as_inner_mut().as_inner_mut().h_left_clicked {
        let mut w2 = this.clone().upcast::<Widget>();
		let mut w2 = common::cast_gtk_widget_to_member_mut::<Button>(&mut w2).unwrap();
		(cb.as_mut())(w2);
    }
}

impl_all_defaults!(Button);

