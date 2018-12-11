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
            let self_widget: gtk::Widget = fr.as_inner_mut().as_inner_mut().as_inner_mut().base.widget.clone().into();
            let frame = self_widget.downcast::<GtkFrameSys>().unwrap();
            frame.set_label(label);
        }
        fr.as_inner_mut().as_inner_mut().as_inner_mut().base.widget.connect_size_allocate(on_size_allocate);
        fr
    }
}

impl SingleContainerInner for GtkFrame {
    fn set_child(&mut self, base: &mut MemberBase, mut child: Option<Box<dyn controls::Control>>) -> Option<Box<dyn controls::Control>> {
        let mut old = self.child.take();
        let (pw, ph) = self.size();
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
                new.on_added_to_container(
                    self2,
                    0,
                    0,
                    utils::coord_to_size(cmp::max(0, pw as i32 - self.base.widget.get_margin_start() - self.base.widget.get_margin_end())),
                    utils::coord_to_size(cmp::max(0, ph as i32 - self.base.widget.get_margin_top() - self.base.widget.get_margin_bottom())),
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
        let self_widget: gtk::Widget = self.base.widget.clone().into();
        Cow::Owned(self_widget.downcast::<GtkFrameSys>().unwrap().get_label().unwrap_or(String::new()))
    }
    fn set_label(&mut self, _: &mut MemberBase, label: &str) {
        let self_widget: gtk::Widget = self.base.widget.clone().into();
        self_widget.downcast::<GtkFrameSys>().unwrap().set_label(label)
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
        self.draw(member, control, Some((x, y)));
        if let Some(ref mut child) = self.child {
            let self2 = unsafe { utils::base_to_impl_mut::<Frame>(member) };
            child.on_added_to_container(
                self2,
                0,
                0,
                utils::coord_to_size(cmp::max(0, pw as i32 - self.base.widget.get_margin_start() - self.base.widget.get_margin_end())),
                utils::coord_to_size(cmp::max(0, ph as i32 - self.base.widget.get_margin_top() - self.base.widget.get_margin_bottom())),
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
    fn draw(&mut self, member: &mut MemberBase, control: &mut ControlBase, coords: Option<(i32, i32)>) {
        self.base.draw(member, control, coords);
    }
    fn measure(&mut self, member: &mut MemberBase, control: &mut ControlBase, parent_width: u16, parent_height: u16) -> (u16, u16, bool) {
        let old_size = self.base.measured_size;
        self.base.measured_size = match member.visibility {
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
                        if label_size.0 < 0 {
                            let self_widget: gtk::Widget = self.base.widget.clone().into();
                            let frame_sys = self_widget.downcast::<GtkFrameSys>().unwrap();
                            let label = frame_sys.get_label_widget().unwrap().downcast::<Label>().unwrap();
                            label_size = label.get_layout().unwrap().get_pixel_size();
                        }
                        size + label_size.0 + self.base.widget.get_margin_start() + self.base.widget.get_margin_end()
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
                        if label_size.1 < 0 {
                            let self_widget: gtk::Widget = self.base.widget.clone().into();
                            let frame_sys = self_widget.downcast::<GtkFrameSys>().unwrap();
                            let label = frame_sys.get_label_widget().unwrap().downcast::<Label>().unwrap();
                            label_size = label.get_layout().unwrap().get_pixel_size();
                        }
                        size + label_size.1 + self.base.widget.get_margin_top() + self.base.widget.get_margin_bottom() + 2 // TODO WHY???
                    }
                };
                (cmp::max(0, w) as u16, cmp::max(0, h) as u16)
            }
        };
        (self.base.measured_size.0, self.base.measured_size.1, self.base.measured_size != old_size)
    }
    fn invalidate(&mut self, _: &mut MemberBase, _: &mut ControlBase) {
        self.base.invalidate()
    }
}

#[allow(dead_code)]
pub(crate) fn spawn() -> Box<dyn controls::Control> {
    Frame::with_label("").into_control()
}

fn on_size_allocate(this: &::gtk::Widget, _allo: &::gtk::Rectangle) {
    let mut ll = this.clone().upcast::<Widget>();
    let ll = common::cast_gtk_widget_to_member_mut::<Frame>(&mut ll).unwrap();

    let measured_size = ll.as_inner().as_inner().as_inner().base.measured_size;
    ll.call_on_resize(measured_size.0 as u16, measured_size.1 as u16);
}

impl_all_defaults!(Frame);
