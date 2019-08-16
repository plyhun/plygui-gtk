use crate::common::{self, *};

use gtk::{ProgressBar as GtkProgressBarSys, ProgressBarExt, WidgetExt};
pub type ProgressBar = Member<Control<GtkProgressBar>>;

#[repr(C)]
pub struct GtkProgressBar {
    base: GtkControlBase<ProgressBar>,
}

impl HasProgressInner for GtkProgressBar {
	fn progress(&self, _base: &MemberBase) -> types::Progress {
	    let self_widget: Object = Object::from(self.base.widget.clone()).into();
        let progress_bar = self_widget.downcast::<GtkProgressBarSys>().unwrap();
        if progress_bar.get_inverted() {
            return types::Progress::None;
        }
        if progress_bar.get_pulse_step() > 0.0 {
            return types::Progress::Undefined;
        }
        types::Progress::Value(
            (progress_bar.get_fraction() * 100.0) as u32,
            100
        )
    }
	fn set_progress(&mut self, _base: &mut MemberBase, arg: types::Progress) {
	    let self_widget: Object = Object::from(self.base.widget.clone()).into();
        let progress_bar = self_widget.downcast::<GtkProgressBarSys>().unwrap();
        match arg {
        	types::Progress::Value(current, total) => {
        	    progress_bar.set_inverted(false);
        		progress_bar.set_pulse_step(0.0);
        		progress_bar.set_fraction(1.0 / total as f64 * current as f64);
        	},
        	types::Progress::Undefined => {
        	    progress_bar.set_inverted(false);
        		progress_bar.set_pulse_step(0.1);
        		progress_bar.pulse();
        	},
        	types::Progress::None => {
        	    progress_bar.set_inverted(true);
        	    progress_bar.set_fraction(0.0);
        	}
        }
	}
}
impl ProgressBarInner for GtkProgressBar {
    fn with_progress(arg: types::Progress) -> Box<ProgressBar> {
        use crate::plygui_api::controls::HasProgress;
        
        let mut pb = Box::new(Member::with_inner(
            Control::with_inner(
                GtkProgressBar {
                    base: common::GtkControlBase::with_gtk_widget(reckless::progress_bar::RecklessProgressBar::new().upcast::<Widget>()),
                },
                (),
            ),
            MemberFunctions::new(_as_any, _as_any_mut, _as_member, _as_member_mut),
        ));
        {
            let ptr = pb.as_ref() as *const _ as *mut std::os::raw::c_void;
            pb.as_inner_mut().as_inner_mut().base.set_pointer(ptr);
        }
        {
            let self_widget: Object = Object::from(pb.as_inner_mut().as_inner_mut().base.widget.clone()).into();
            let progress_bar = self_widget.downcast::<GtkProgressBarSys>().unwrap();
            progress_bar.set_show_text(false);
        }
        Object::from(pb.as_inner_mut().as_inner_mut().base.widget.clone()).downcast::<Widget>().unwrap().connect_size_allocate(on_size_allocate);
        pb.set_progress(arg);
        pb
    }
}

impl HasLayoutInner for GtkProgressBar {
    fn on_layout_changed(&mut self, _base: &mut MemberBase) {
        self.base.invalidate();
    }
}

impl ControlInner for GtkProgressBar {
    fn on_added_to_container(&mut self, member: &mut MemberBase, control: &mut ControlBase, _parent: &dyn controls::Container, x: i32, y: i32, pw: u16, ph: u16) {
        self.measure(member, control, pw, ph);
        control.coords = Some((x, y));
        self.draw(member, control);
    }
    fn on_removed_from_container(&mut self, _: &mut MemberBase, _: &mut ControlBase, _: &dyn controls::Container) {}

    fn parent(&self) -> Option<&dyn controls::Member> {
        self.base.parent().map(|m| m.as_member())
    }
    fn parent_mut(&mut self) -> Option<&mut dyn controls::Member> {
        self.base.parent_mut().map(|m| m.as_member_mut())
    }
    fn root(&self) -> Option<&dyn controls::Member> {
        self.base.root().map(|m| m.as_member())
    }
    fn root_mut(&mut self) -> Option<&mut dyn controls::Member> {
        self.base.root_mut().map(|m| m.as_member_mut())
    }

    #[cfg(feature = "markup")]
    fn fill_from_markup(&mut self, member: &mut MemberBase, control: &mut ControlBase, mberarkup: &super::markup::Markup, registry: &mut super::markup::MarkupRegistry) {
        use plygui_api::markup::MEMBER_TYPE_PROGRESS_BAR;
        fill_from_markup_base!(self, base, markup, registry, ProgressBar, [MEMBER_TYPE_PROGRESS_BAR]);
        fill_from_markup_label!(self, &mut base.member, markup);
        fill_from_markup_callbacks!(self, markup, registry, [on_click => plygui_api::callbacks::Click]);
    }
}

impl HasNativeIdInner for GtkProgressBar {
    type Id = common::GtkWidget;

    unsafe fn native_id(&self) -> Self::Id {
        self.base.widget.clone().into()
    }
}

impl HasSizeInner for GtkProgressBar {
    fn on_size_set(&mut self, _: &mut MemberBase, (width, height): (u16, u16)) -> bool {
        self.base.widget().set_size_request(width as i32, height as i32);
        true
    }
}

impl HasVisibilityInner for GtkProgressBar {
    fn on_visibility_set(&mut self, _: &mut MemberBase, _: types::Visibility) -> bool {
        self.base.invalidate()
    }
}

impl MemberInner for GtkProgressBar {}

impl Drawable for GtkProgressBar {
    fn draw(&mut self, _: &mut MemberBase, control: &mut ControlBase) {
        self.base.draw(control);
    }
    fn measure(&mut self, _: &mut MemberBase, control: &mut ControlBase, parent_width: u16, parent_height: u16) -> (u16, u16, bool) {
        let old_size = control.measured;
        control.measured = match control.visibility {
            types::Visibility::Gone => (0, 0),
            _ => {
                let w = match control.layout.width {
                    layout::Size::MatchParent => parent_width as i32,
                    layout::Size::Exact(w) => w as i32,
                    layout::Size::WrapContent => {
                        let widget: Object = self.base.widget.clone().into();
                        let widget = widget.downcast::<Widget>().unwrap();
                        24 + widget.get_margin_start() + widget.get_margin_end() + DEFAULT_PADDING + DEFAULT_PADDING
                    }
                };
                let h = match control.layout.height {
                    layout::Size::MatchParent => parent_height as i32,
                    layout::Size::Exact(h) => h as i32,
                    layout::Size::WrapContent => {
                        let widget: Object = self.base.widget.clone().into();
                        let widget = widget.downcast::<Widget>().unwrap();
                        24 + widget.get_margin_top() + widget.get_margin_bottom() + DEFAULT_PADDING + DEFAULT_PADDING
                    }
                };
                (cmp::max(0, w) as u16, cmp::max(0, h) as u16)
            }
        };
        (control.measured.0, control.measured.1, control.measured != old_size)
    }
    fn invalidate(&mut self, _: &mut MemberBase, _: &mut ControlBase) {
        self.base.invalidate();
    }
}

#[allow(dead_code)]
pub(crate) fn spawn() -> Box<dyn controls::Control> {
    ProgressBar::with_progress(types::Progress::None).into_control()
}

fn on_size_allocate(this: &::gtk::Widget, _allo: &::gtk::Rectangle) {
    let mut ll = this.clone().upcast::<Widget>();
    let ll = common::cast_gtk_widget_to_member_mut::<ProgressBar>(&mut ll).unwrap();

    let measured_size = ll.as_inner().base().measured;
    ll.call_on_size(measured_size.0 as u16, measured_size.1 as u16);
}

default_impls_as!(ProgressBar);
