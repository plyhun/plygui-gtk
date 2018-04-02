use super::*;

use gtk::{Cast, Widget, WidgetExt, Button as GtkButton, ButtonExt, Bin, BinExt, Label, LabelExt, Fixed, FixedExt, Rectangle, CssProvider, CssProviderExt, StyleContextExt};
use pango::LayoutExt;

use plygui_api::{layout, types, development, callbacks};
use plygui_api::traits::{UiControl, UiHasLayout, UiHasLabel, UiButton, UiMember, UiContainer, UiClickable};
use plygui_api::members::MEMBER_ID_BUTTON;

use std::borrow::Cow;
use std::cmp::max;

const DEFAULT_PADDING: i32 = 6;

#[repr(C)]
pub struct Button {
    base: common::GtkControlBase,

    h_left_clicked: Option<callbacks::Click>,
    h_right_clicked: Option<callbacks::Click>,
}

impl Button {
    pub fn new(label: &str) -> Box<Button> {
        let mut btn = Box::new(Button {
                     base: common::GtkControlBase::with_params(
		                     	GtkButton::new_with_label(label).upcast::<Widget>(),
		                     	invalidate_impl,
                             	development::UiMemberFunctions {
		                             fn_member_id: member_id,
								     fn_is_control: is_control,
								     fn_is_control_mut: is_control_mut,
								     fn_size: size,
	                            },
                             ),
                     h_left_clicked: None,
                     h_right_clicked: None,
                 });
        {
        	let ptr = btn.as_ref() as *const _ as *mut std::os::raw::c_void;
        	btn.base.set_pointer(ptr);
        }
        {
        	let button = btn.base.widget.clone().downcast::<GtkButton>().unwrap();
			button.connect_clicked(on_click);
        }
        btn.set_layout_padding(layout::BoundarySize::AllTheSame(DEFAULT_PADDING).into());
        btn.base.widget.connect_size_allocate(on_size_allocate);
        btn
    }
    fn apply_padding(&mut self) {
	    let (lp,tp,rp,bp) = self.base.control_base.layout.padding.into();
			
	    let btn = self.base.widget.clone().downcast::<GtkButton>().unwrap();   
		let css = CssProvider::new();
		css.load_from_data(format!("GtkButton {{ padding-left: {}px; padding-top: {}px; padding-right: {}px; padding-bottom: {}px; }}", lp, tp, rp, bp).as_bytes()).unwrap();
		btn.get_style_context().unwrap().add_provider(&css, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);
    }
}

impl UiHasLabel for Button {
	fn label<'a>(&'a self) -> Cow<'a, str> {
		Cow::Owned(self.base.widget.clone().downcast::<GtkButton>().unwrap().get_label().unwrap_or(String::new()))
	}
    fn set_label(&mut self, label: &str) {
    	self.base.widget.clone().downcast::<GtkButton>().unwrap().set_label(label)
    }
}

impl UiClickable for Button {
	fn on_click(&mut self, cb: Option<callbacks::Click>) {
		self.h_left_clicked = cb;
    }    
}

impl UiButton for Button {
    
    /*fn on_right_click(&mut self, cb: Option<Box<FnMut(&mut UiButton)>>) {
        self.h_right_clicked = cb;
    }*/
    
    fn as_control(&self) -> &UiControl {
    	self
    }
	fn as_control_mut(&mut self) -> &mut UiControl {
		self
	}
	fn as_clickable(&self) -> &UiClickable {
		self
	}
	fn as_clickable_mut(&mut self) -> &mut UiClickable {
		self
	}
	fn as_has_label(&self) -> &UiHasLabel {
		self
	}
	fn as_has_label_mut(&mut self) -> &mut UiHasLabel {
		self
	}
}

impl UiHasLayout for Button {
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
		self.apply_padding();
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

impl UiControl for Button {
    fn is_container_mut(&mut self) -> Option<&mut UiContainer> {
        None
    }
    fn is_container(&self) -> Option<&UiContainer> {
        None
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
        self.base.dirty = false;
        self.draw(Some((x, y)));
    }
    fn on_removed_from_container(&mut self, _: &UiContainer) {}	
    
    #[cfg(feature = "markup")]
    fn fill_from_markup(&mut self, markup: &plygui_api::markup::Markup, registry: &mut plygui_api::markup::MarkupRegistry) {
    	use plygui_api::markup::MEMBER_TYPE_BUTTON;
    	
    	fill_from_markup_base!(self, markup, registry, Button, [MEMBER_ID_BUTTON, MEMBER_TYPE_BUTTON]);
    	fill_from_markup_label!(self, markup);
    	//fill_from_markup_callbacks!(self, markup, registry, ["on_click" => FnMut(&mut UiButton)]);
    	
    	if let Some(on_click) = markup.attributes.get("on_click") {
    		let callback: callbacks::Click = registry.pop_callback(on_click.as_attribute()).unwrap();
    		self.on_click(Some(callback));
    	}
    }
    fn as_has_layout(&self) -> &UiHasLayout {
    	self
    }
	fn as_has_layout_mut(&mut self) -> &mut UiHasLayout {
		self
	}
}

impl UiMember for Button {
    fn set_visibility(&mut self, visibility: types::Visibility) {
        self.base.set_visibility(visibility);
        self.base.invalidate();
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

impl development::UiDrawable for Button {
	fn draw(&mut self, coords: Option<(i32, i32)>) {
    	if coords.is_some() {
    		self.base.coords = coords;
    	}
    	if let Some(coords) = self.base.coords {
			let (lm,tm,rm,bm) = self.base.control_base.layout.margin.into();
	        self.base.widget.get_parent().unwrap().downcast::<Fixed>().unwrap().move_(&self.base.widget, coords.0 as i32 + lm, coords.1 as i32 + tm);
			self.base.widget.set_size_request(self.base.measured_size.0 as i32 - lm - rm, self.base.measured_size.1 as i32 - rm - bm);
			if let types::Visibility::Visible = self.base.control_base.member_base.visibility {
				self.base.widget.show();
			} else {
				self.base.widget.hide();
			}
		}
        self.base.dirty = false;
    }
    fn measure(&mut self, parent_width: u16, parent_height: u16) -> (u16, u16, bool) {
    	let old_size = self.base.measured_size;
    	self.base.measured_size = match self.visibility() {
            types::Visibility::Gone => (0, 0),
            _ => {
                let (lp,tp,rp,bp) = self.base.control_base.layout.padding.into();
		    	let (lm,tm,rm,bm) = self.base.control_base.layout.margin.into();
		    	    	
		    	let mut label_size = (-1i32, -1i32);
                let w = match self.base.control_base.layout.width {
                    layout::Size::MatchParent => parent_width as i32,
                    layout::Size::Exact(w) => w as i32,
                    layout::Size::WrapContent => {
                        if label_size.0 < 0 {
                        	let mut bin = self.base.widget.clone().downcast::<Bin>().unwrap();
                        	let mut label = bin.get_child().unwrap().downcast::<Label>().unwrap();		
                        	label_size = label.get_layout().unwrap().get_pixel_size();			
                        }
                        // why the bloody hell I need these?
                        label_size.0 += 4;
                        label_size.1 += 4;
                        
                        label_size.0 + lp + rp + lm + rm
                    } 
                };
                let h = match self.base.control_base.layout.height {
                    layout::Size::MatchParent => parent_height as i32,
                    layout::Size::Exact(h) => h as i32,
                    layout::Size::WrapContent => {
                        if label_size.1 < 0 {
                            let mut bin = self.base.widget.clone().downcast::<Bin>().unwrap();
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
    	self.base.dirty = self.base.measured_size != old_size;
        (
            self.base.measured_size.0,
            self.base.measured_size.1,
            self.base.dirty,
        )
    }
}

#[allow(dead_code)]
pub(crate) fn spawn() -> Box<UiControl> {
	Button::new("")
}

impl_invalidate!(Button);
impl_is_control!(Button);
impl_size!(Button);
impl_member_id!(MEMBER_ID_BUTTON);
impl_on_size_allocate!(Button);

fn on_click(this: &GtkButton) {
	let mut b = this.clone().upcast::<Widget>();
	let b = common::cast_gtk_widget_to_uimember_mut::<Button>(&mut b).unwrap();
	if let Some(ref mut cb) = b.h_left_clicked {
        let mut w2 = this.clone().upcast::<Widget>();
		let mut w2 = common::cast_gtk_widget_to_uimember_mut::<Button>(&mut w2).unwrap();
		(cb.as_mut())(w2);
    }
}

