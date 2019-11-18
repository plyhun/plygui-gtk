use crate::common::{self, *};

use gtk::{Bin, BinExt, Button as GtkButtonSys, ButtonExt, Label, LabelExt};
use gdk::ModifierType;
use pango::LayoutExt;

use std::borrow::Cow;

pub type Button = Member<Control<GtkButton>>;

#[repr(C)]
pub struct GtkButton {
    base: GtkControlBase<Button>,

    h_left_clicked: Option<callbacks::OnClick>,
    h_right_clicked: Option<callbacks::OnClick>,
    skip_callbacks: bool,
}

impl ButtonInner for GtkButton {
    fn with_label(label: &str) -> Box<Button> {
        let mut btn = Box::new(Member::with_inner(
            Control::with_inner(
                GtkButton {
                    base: common::GtkControlBase::with_gtk_widget(reckless::RecklessButton::new().upcast::<Widget>()),
                    h_left_clicked: None,
                    h_right_clicked: None,
                    skip_callbacks: false,
                },
                (),
            ),
            MemberFunctions::new(_as_any, _as_any_mut, _as_member, _as_member_mut),
        ));
        {
            let ptr = btn.as_ref() as *const _ as *mut std::os::raw::c_void;
            btn.as_inner_mut().as_inner_mut().base.set_pointer(ptr);
        }
        {
            let self_widget: Object = Object::from(btn.as_inner_mut().as_inner_mut().base.widget.clone()).into();
            let button = self_widget.downcast::<GtkButtonSys>().unwrap();
            button.set_label(label);
            button.connect_clicked(on_click);
        }
        Object::from(btn.as_inner_mut().as_inner_mut().base.widget.clone()).downcast::<Widget>().unwrap().connect_size_allocate(on_size_allocate);
        btn
    }
}

impl HasLabelInner for GtkButton {
    fn label(&self, _: &MemberBase) -> Cow<str> {
        let self_widget: Object = Object::from(self.base.widget.clone()).into();
        Cow::Owned(self_widget.downcast::<GtkButtonSys>().unwrap().get_label().unwrap_or(String::new()))
    }
    fn set_label(&mut self, _: &mut MemberBase, label: Cow<str>) {
        let self_widget: Object = Object::from(self.base.widget.clone()).into();
        self_widget.downcast::<GtkButtonSys>().unwrap().set_label(&label)
    }
}

impl ClickableInner for GtkButton {
    fn on_click(&mut self, cb: Option<callbacks::OnClick>) {
        self.h_left_clicked = cb;
    }
    fn click(&mut self, skip_callbacks: bool) {
        self.skip_callbacks = skip_callbacks;
        let self_widget: Object = Object::from(self.base.widget.clone()).into();
        gtk::test_widget_click(&self_widget.downcast::<GtkButtonSys>().unwrap(), 1, ModifierType::BUTTON1_MASK);
    }
}

impl HasLayoutInner for GtkButton {
    fn on_layout_changed(&mut self, _base: &mut MemberBase) {
        self.base.invalidate();
    }
}

impl ControlInner for GtkButton {
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
        use plygui_api::markup::MEMBER_TYPE_BUTTON;
        fill_from_markup_base!(self, base, markup, registry, Button, [MEMBER_TYPE_BUTTON]);
        fill_from_markup_label!(self, &mut base.member, markup);
        fill_from_markup_callbacks!(self, markup, registry, [on_click => plygui_api::callbacks::Click]);
    }
}

impl HasNativeIdInner for GtkButton {
    type Id = common::GtkWidget;

    unsafe fn native_id(&self) -> Self::Id {
        self.base.widget.clone().into()
    }
}

impl HasSizeInner for GtkButton {
    fn on_size_set(&mut self, _: &mut MemberBase, (width, height): (u16, u16)) -> bool {
        self.base.widget().set_size_request(width as i32, height as i32);
        true
    }
}

impl HasVisibilityInner for GtkButton {
    fn on_visibility_set(&mut self, _: &mut MemberBase, _: types::Visibility) -> bool {
        self.base.invalidate()
    }
}

impl MemberInner for GtkButton {}

impl Drawable for GtkButton {
    fn draw(&mut self, _: &mut MemberBase, control: &mut ControlBase) {
        self.base.draw(control);
    }
    fn measure(&mut self, _: &mut MemberBase, control: &mut ControlBase, parent_width: u16, parent_height: u16) -> (u16, u16, bool) {
        let old_size = control.measured;
        control.measured = match control.visibility {
            types::Visibility::Gone => (0, 0),
            _ => {
                let mut label_size = (-1i32, -1i32);

                let w = match control.layout.width {
                    layout::Size::MatchParent => parent_width as i32,
                    layout::Size::Exact(w) => w as i32,
                    layout::Size::WrapContent => {
                        let widget: Object = self.base.widget.clone().into();
                        if label_size.0 < 0 {
                            let bin = widget.clone().downcast::<Bin>().unwrap();
                            let label = bin.get_child().unwrap().downcast::<Label>().unwrap();
                            label_size = label.get_layout().unwrap().get_pixel_size();
                        }
                        let widget = widget.downcast::<Widget>().unwrap();
                        label_size.0 + widget.get_margin_start() + widget.get_margin_end() + DEFAULT_PADDING + DEFAULT_PADDING
                    }
                };
                let h = match control.layout.height {
                    layout::Size::MatchParent => parent_height as i32,
                    layout::Size::Exact(h) => h as i32,
                    layout::Size::WrapContent => {
                        let widget: Object = self.base.widget.clone().into();
                        if label_size.1 < 0 {
                            let bin = widget.clone().downcast::<Bin>().unwrap();
                            let label = bin.get_child().unwrap().downcast::<Label>().unwrap();
                            label_size = label.get_layout().unwrap().get_pixel_size();
                        }
                        let widget = widget.downcast::<Widget>().unwrap();
                        label_size.1 + widget.get_margin_top() + widget.get_margin_bottom() + DEFAULT_PADDING + DEFAULT_PADDING
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
    Button::with_label("").into_control()
}

fn on_size_allocate(this: &::gtk::Widget, _allo: &::gtk::Rectangle) {
    let mut ll = this.clone().upcast::<Widget>();
    let ll = common::cast_gtk_widget_to_member_mut::<Button>(&mut ll).unwrap();

    let measured_size = ll.as_inner().base().measured;
    ll.call_on_size(measured_size.0 as u16, measured_size.1 as u16);
}

fn on_click(this: &GtkButtonSys) {
    let mut b = this.clone().upcast::<Widget>();
    let b = common::cast_gtk_widget_to_member_mut::<Button>(&mut b).unwrap();
    if let Some(ref mut cb) = b.as_inner_mut().as_inner_mut().h_left_clicked {
        let mut w2 = this.clone().upcast::<Widget>();
        let w2 = common::cast_gtk_widget_to_member_mut::<Button>(&mut w2).unwrap();
        (cb.as_mut())(w2);
    }
}

default_impls_as!(Button);
