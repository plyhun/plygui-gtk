use super::common::*;
use super::*;

use gtk::{Cast, ContainerExt, Frame as GtkFrameSys, FrameExt, Label, LabelExt, Widget, WidgetExt};
use pango::LayoutExt;

use std::borrow::Cow;

pub type Frame = Member<Control<SingleContainer<GtkFrame>>>;

#[repr(C)]
pub struct GtkFrame {
    base: common::GtkControlBase<Frame>,
    child: Option<Box<dyn controls::Control>>,
}

impl FrameInner for GtkFrame {
    fn with_label(label: &str) -> Box<Frame> {
        let mut fr = Box::new(Member::with_inner(
            Control::with_inner(
                SingleContainer::with_inner(
                    GtkFrame {
                        base: common::GtkControlBase::with_gtk_widget(reckless::frame::RecklessFrame::new().upcast::<Widget>()),
                        child: None,
                    },
                    (),
                ),
                (),
            ),
            MemberFunctions::new(_as_any, _as_any_mut, _as_member, _as_member_mut),
        ));
        {
            let ptr = fr.as_ref() as *const _ as *mut std::os::raw::c_void;
            fr.as_inner_mut().as_inner_mut().as_inner_mut().base.set_pointer(ptr);
        }
        {
            let frame = Object::from(fr.as_inner_mut().as_inner_mut().as_inner_mut().base.widget.clone()).downcast::<GtkFrameSys>().unwrap();
            frame.set_label(label);
        }
        Object::from(fr.as_inner_mut().as_inner_mut().as_inner_mut().base.widget.clone()).downcast::<Widget>().unwrap().connect_size_allocate(on_size_allocate);
        fr
    }
}

impl SingleContainerInner for GtkFrame {
    fn set_child(&mut self, base: &mut MemberBase, mut child: Option<Box<dyn controls::Control>>) -> Option<Box<dyn controls::Control>> {
        let mut old = self.child.take();
        let this = unsafe { utils::base_to_impl_mut::<Frame>(base) };
        let (pw, ph) = this.as_inner().base().measured;
        let frame_sys = Object::from(self.base.widget.clone()).downcast::<GtkFrameSys>().unwrap();
        if let Some(old) = old.as_mut() {
            let old_sys: common::GtkWidget = unsafe { old.native_id() }.into();
            frame_sys.remove(&Object::from(old_sys).downcast::<Widget>().unwrap());
            if this.as_inner().base().coords.is_some() {
                old.on_removed_from_container(this);
            }
        }
        if let Some(new) = child.as_mut() {
            let widget = common::cast_control_to_gtkwidget(new.as_ref());
            frame_sys.add(&Object::from(widget).downcast::<Widget>().unwrap());
            let self_widget = Object::from(self.base.widget.clone()).downcast::<Widget>().unwrap();
            if this.as_inner().base().coords.is_some() {
                new.on_added_to_container(
                    this,
                    0,
                    0,
                    utils::coord_to_size(cmp::max(0, pw as i32 - self_widget.get_margin_start() - self_widget.get_margin_end())),
                    utils::coord_to_size(cmp::max(0, ph as i32 - self_widget.get_margin_top() - self_widget.get_margin_bottom())),
                );
            }
        }
        self.child = child;
        self.base.invalidate();

        old
    }
    fn child(&self) -> Option<&dyn controls::Control> {
        self.child.as_ref().map(|c| c.as_ref())
    }
    fn child_mut(&mut self) -> Option<&mut dyn controls::Control> {
        if let Some(child) = self.child.as_mut() {
            Some(child.as_mut())
        } else {
            None
        }
    }
}

impl ContainerInner for GtkFrame {
    fn find_control_by_id_mut(&mut self, id: ids::Id) -> Option<&mut dyn controls::Control> {
        if let Some(child) = self.child.as_mut() {
            if child.as_member().id() == id {
                Some(child.as_mut())
            } else if let Some(c) = child.is_container_mut() {
                c.find_control_by_id_mut(id)
            } else {
                None
            }
        } else {
            None
        }
    }
    fn find_control_by_id(&self, id: ids::Id) -> Option<&dyn controls::Control> {
        if let Some(child) = self.child.as_ref() {
            if child.as_member().id() == id {
                Some(child.as_ref())
            } else if let Some(c) = child.is_container() {
                c.find_control_by_id(id)
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl HasLabelInner for GtkFrame {
    fn label<'a>(&'a self) -> Cow<'a, str> {
        Cow::Owned(Object::from(self.base.widget.clone()).downcast::<GtkFrameSys>().unwrap().get_label().unwrap_or(String::new()))
    }
    fn set_label(&mut self, _: &mut MemberBase, label: &str) {
        Object::from(self.base.widget.clone()).downcast::<GtkFrameSys>().unwrap().set_label(label)
    }
}

impl HasLayoutInner for GtkFrame {
    fn on_layout_changed(&mut self, _base: &mut MemberBase) {
        self.base.invalidate();
    }
}

impl ControlInner for GtkFrame {
    fn on_added_to_container(&mut self, member: &mut MemberBase, control: &mut ControlBase, _parent: &dyn controls::Container, x: i32, y: i32, pw: u16, ph: u16) {
        self.measure(member, control, pw, ph);
        control.coords = Some((x, y));
        self.draw(member, control);
        if let Some(ref mut child) = self.child {
            let self2 = unsafe { utils::base_to_impl_mut::<Frame>(member) };
            let self_widget = Object::from(self.base.widget.clone()).downcast::<Widget>().unwrap();
            child.on_added_to_container(
                self2,
                0,
                0,
                utils::coord_to_size(cmp::max(0, pw as i32 - self_widget.get_margin_start() - self_widget.get_margin_end())),
                utils::coord_to_size(cmp::max(0, ph as i32 - self_widget.get_margin_top() - self_widget.get_margin_bottom())),
            );
        }
    }
    fn on_removed_from_container(&mut self, member: &mut MemberBase, _control: &mut ControlBase, _: &dyn controls::Container) {
        if let Some(ref mut child) = self.child {
            let self2 = unsafe { utils::base_to_impl_mut::<Frame>(member) };
            child.on_removed_from_container(self2);
        }
    }

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
        use plygui_api::markup::MEMBER_TYPE_FRAME;
        fill_from_markup_base!(self, member, markup, registry, Frame, [MEMBER_TYPE_FRAME]);
        fill_from_markup_label!(self, member, markup);
        fill_from_markup_child!(self, member, markup, registry);
    }
}

impl HasNativeIdInner for GtkFrame {
    type Id = common::GtkWidget;

    unsafe fn native_id(&self) -> Self::Id {
        self.base.widget.clone().into()
    }
}

impl HasSizeInner for GtkFrame {
    fn on_size_set(&mut self, _: &mut MemberBase, (width, height): (u16, u16)) -> bool {
        Object::from(self.base.widget.clone()).downcast::<Widget>().unwrap().set_size_request(width as i32, height as i32);
        true
    }
}

impl HasVisibilityInner for GtkFrame {
    fn on_visibility_set(&mut self, _: &mut MemberBase, _: types::Visibility) -> bool {
        self.base.invalidate()
    }
}

impl MemberInner for GtkFrame {}

impl Drawable for GtkFrame {
    fn draw(&mut self, _: &mut MemberBase, control: &mut ControlBase) {
        self.base.draw(control);
    }
    fn measure(&mut self, _: &mut MemberBase, control: &mut ControlBase, parent_width: u16, parent_height: u16) -> (u16, u16, bool) {
        let old_size = control.measured;
        control.measured = match control.visibility {
            types::Visibility::Gone => (0, 0),
            _ => {
                let mut label_size = (-1i32, -1i32);
                let mut measured = false;
                let w = match control.layout.width {
                    layout::Size::MatchParent => parent_width as i32,
                    layout::Size::Exact(w) => w as i32,
                    layout::Size::WrapContent => {
                        let mut size = 0;
                        if let Some(ref mut child) = self.child {
                            let (cw, _, _) = child.measure(cmp::max(0, parent_width as i32) as u16, cmp::max(0, parent_height as i32) as u16);
                            size += cw as i32;
                            measured = true;
                        }
                        let self_widget = Object::from(self.base.widget.clone()).downcast::<Widget>().unwrap();
                        if label_size.0 < 0 {
                            let frame_sys = self_widget.clone().downcast::<GtkFrameSys>().unwrap();
                            let label = frame_sys.get_label_widget().unwrap().downcast::<Label>().unwrap();
                            label_size = label.get_layout().unwrap().get_pixel_size();
                        }
                        size + label_size.0 + self_widget.get_margin_start() + self_widget.get_margin_end()
                    }
                };
                let h = match control.layout.height {
                    layout::Size::MatchParent => parent_height as i32,
                    layout::Size::Exact(h) => h as i32,
                    layout::Size::WrapContent => {
                        let mut size = 0;
                        if let Some(ref mut child) = self.child {
                            let ch = if measured {
                                child.size().1
                            } else {
                                let (_, ch, _) = child.measure(cmp::max(0, parent_width as i32) as u16, cmp::max(0, parent_height as i32) as u16);
                                ch
                            };
                            size += ch as i32;
                        }
                        let self_widget = Object::from(self.base.widget.clone()).downcast::<Widget>().unwrap();
                        if label_size.1 < 0 {
                            let frame_sys = self_widget.clone().downcast::<GtkFrameSys>().unwrap();
                            let label = frame_sys.get_label_widget().unwrap().downcast::<Label>().unwrap();
                            label_size = label.get_layout().unwrap().get_pixel_size();
                        }
                        size + label_size.1 + self_widget.get_margin_top() + self_widget.get_margin_bottom() + 2 // TODO WHY???
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
    Frame::with_label("").into_control()
}

fn on_size_allocate(this: &::gtk::Widget, _allo: &::gtk::Rectangle) {
    let mut ll = this.clone().upcast::<Widget>();
    let ll = common::cast_gtk_widget_to_member_mut::<Frame>(&mut ll).unwrap();

    let measured_size = ll.as_inner().base().measured;
    ll.call_on_size(measured_size.0 as u16, measured_size.1 as u16);
}

impl_all_defaults!(Frame);
