use crate::common::{self, *};

use gtk::{Box as GtkBox, BoxExt, Cast, ContainerExt, OrientableExt, Widget, WidgetExt};

pub type LinearLayout = Member<Control<MultiContainer<GtkLinearLayout>>>;

#[repr(C)]
pub struct GtkLinearLayout {
    base: common::GtkControlBase<LinearLayout>,
    children: Vec<Box<dyn controls::Control>>,
}

impl LinearLayoutInner for GtkLinearLayout {
    fn with_orientation(orientation: layout::Orientation) -> Box<LinearLayout> {
        let mut ll = Box::new(Member::with_inner(
            Control::with_inner(
                MultiContainer::with_inner(
                    GtkLinearLayout {
                        base: common::GtkControlBase::with_gtk_widget(reckless::boxc::RecklessBox::new().upcast::<Widget>()),
                        children: Vec::new(),
                    },
                    (),
                ),
                (),
            ),
            MemberFunctions::new(_as_any, _as_any_mut, _as_member, _as_member_mut),
        ));
        {
            let ptr = ll.as_ref() as *const _ as *mut std::os::raw::c_void;
            ll.as_inner_mut().as_inner_mut().as_inner_mut().base.set_pointer(ptr);
        }
        {
            let boxc = Object::from(ll.as_inner_mut().as_inner_mut().as_inner_mut().base.widget.clone()).downcast::<GtkBox>().unwrap();
            boxc.set_orientation(common::orientation_to_gtk(orientation));
            boxc.set_spacing(0);
        }
        ll.as_inner_mut().as_inner_mut().as_inner_mut().base.widget().connect_size_allocate(on_size_allocate);
        ll
    }
}

impl HasNativeIdInner for GtkLinearLayout {
    type Id = common::GtkWidget;

    unsafe fn native_id(&self) -> Self::Id {
        self.base.widget.clone().into()
    }
}

impl HasSizeInner for GtkLinearLayout {
    fn on_size_set(&mut self, _: &mut MemberBase, (width, height): (u16, u16)) -> bool {
        self.base.widget().set_size_request(width as i32, height as i32);
        true
    }
}

impl HasVisibilityInner for GtkLinearLayout {
    fn on_visibility_set(&mut self, _: &mut MemberBase, _: types::Visibility) -> bool {
        self.base.invalidate()
    }
}

impl MemberInner for GtkLinearLayout {}

impl Drawable for GtkLinearLayout {
    fn draw(&mut self, _: &mut MemberBase, control: &mut ControlBase) {
        self.base.draw(control);
        for ref mut child in self.children.as_mut_slice() {
            child.draw(Some((0, 0)));
        }
    }
    fn measure(&mut self, _: &mut MemberBase, control: &mut ControlBase, parent_width: u16, parent_height: u16) -> (u16, u16, bool) {
        use std::cmp::max;

        let orientation = self.layout_orientation();
        let old_size = control.measured;
        control.measured = match control.visibility {
            types::Visibility::Gone => (0, 0),
            _ => {
                let mut measured = false;
                let w = match control.layout.width {
                    layout::Size::Exact(w) => w,
                    layout::Size::MatchParent => parent_width,
                    layout::Size::WrapContent => {
                        let mut w = 0;
                        let self_widget = self.base.widget();
                        for ref mut child in self.children.as_mut_slice() {
                            let (cw, _, _) = child.measure(
                                max(0, parent_width as i32 - self_widget.get_margin_start() - self_widget.get_margin_start()) as u16,
                                max(0, parent_height as i32 - self_widget.get_margin_top() - self_widget.get_margin_end()) as u16,
                            );
                            match orientation {
                                layout::Orientation::Horizontal => {
                                    w += cw;
                                }
                                layout::Orientation::Vertical => {
                                    w = max(w, cw);
                                }
                            }
                        }
                        measured = true;
                        max(0, w) as u16
                    }
                };
                let h = match control.layout.height {
                    layout::Size::Exact(h) => h,
                    layout::Size::MatchParent => parent_height,
                    layout::Size::WrapContent => {
                        let mut h = 0;
                        let self_widget = self.base.widget();
                        for ref mut child in self.children.as_mut_slice() {
                            let ch = if measured {
                                child.size().1
                            } else {
                                let (_, ch, _) = child.measure(
                                    max(0, parent_width as i32 - self_widget.get_margin_start() - self_widget.get_margin_end()) as u16,
                                    max(0, parent_height as i32 - self_widget.get_margin_top() - self_widget.get_margin_bottom()) as u16,
                                );
                                ch
                            };
                            match orientation {
                                layout::Orientation::Horizontal => {
                                    h = max(h, ch);
                                }
                                layout::Orientation::Vertical => {
                                    h += ch;
                                }
                            }
                        }
                        max(0, h) as u16
                    }
                };
                (w, h)
            }
        };
        (control.measured.0, control.measured.1, control.measured != old_size)
    }
    fn invalidate(&mut self, _: &mut MemberBase, _: &mut ControlBase) {
        self.base.invalidate();
    }
}

impl HasLayoutInner for GtkLinearLayout {
    fn on_layout_changed(&mut self, _: &mut MemberBase) {
        self.base.invalidate();
    }
}

impl ControlInner for GtkLinearLayout {
    fn on_added_to_container(&mut self, member: &mut MemberBase, control: &mut ControlBase, _parent: &dyn controls::Container, x: i32, y: i32, pw: u16, ph: u16) {
        self.measure(member, control, pw, ph);
        control.coords = Some((x, y));
        self.draw(member, control);
        let (lm, tm, rm, bm) = self.base.margins().into();
        let self2 = self.base.as_control();
        for ref mut child in self.children.as_mut_slice() {
            child.on_added_to_container(self2, 0, 0, utils::coord_to_size(cmp::max(0, pw as i32 - lm - rm)), utils::coord_to_size(cmp::max(0, ph as i32 - tm - bm)));
        }
    }
    fn on_removed_from_container(&mut self, _: &mut MemberBase, _: &mut ControlBase, _: &dyn controls::Container) {
        let self2 = self.base.as_control();
        for mut child in self.children.drain(..) {
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
        use plygui_api::markup::MEMBER_TYPE_LINEAR_LAYOUT;

        fill_from_markup_base!(self, markup, registry, LinearLayout, [MEMBER_TYPE_LINEAR_LAYOUT]);
        fill_from_markup_children!(self, markup, registry);
    }
}

impl HasOrientationInner for GtkLinearLayout {
    fn layout_orientation(&self) -> layout::Orientation {
        let gtk_self = Object::from(self.base.widget.clone()).downcast::<GtkBox>().unwrap();
        common::gtk_to_orientation(gtk_self.get_orientation())
    }
    fn set_layout_orientation(&mut self, _: &mut MemberBase, orientation: layout::Orientation) {
        let gtk_self = Object::from(self.base.widget.clone()).downcast::<GtkBox>().unwrap();
        gtk_self.set_orientation(common::orientation_to_gtk(orientation));
        self.base.invalidate();
    }
}

impl ContainerInner for GtkLinearLayout {
    fn find_control_by_id_mut(&mut self, id: ids::Id) -> Option<&mut dyn controls::Control> {
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
    fn find_control_by_id(&self, id: ids::Id) -> Option<&dyn controls::Control> {
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
}

impl MultiContainerInner for GtkLinearLayout {
    fn len(&self) -> usize {
        self.children.len()
    }
    fn set_child_to(&mut self, base: &mut MemberBase, index: usize, child: Box<dyn controls::Control>) -> Option<Box<dyn controls::Control>> {
        let self2 = unsafe { utils::base_to_impl_mut::<LinearLayout>(base) };

        self.children.insert(index, child);
        let old = if (index + 1) < self.children.len() {
            let mut old = self.children.remove(index + 1);
            if self2.as_inner().base().coords.is_some() {
                old.on_removed_from_container(self2);
            }
            Some(old)
        } else {
            None
        };

        let widget = common::cast_control_to_gtkwidget(self.children.get_mut(index).unwrap().as_mut());
        Object::from(self.base.widget.clone()).downcast::<GtkBox>().unwrap().add::<Widget>(&Object::from(widget).downcast().unwrap());
        if self2.as_inner().base().coords.is_some() {
            let (pw, ph) = self2.as_inner().base().measured;
            let self_widget = self.base.widget();
            self.children.get_mut(index).unwrap().on_added_to_container(
                self2,
                0,
                0,
                utils::coord_to_size(cmp::max(0, pw as i32 - self_widget.get_margin_start() - self_widget.get_margin_end())),
                utils::coord_to_size(cmp::max(0, ph as i32 - self_widget.get_margin_top() - self_widget.get_margin_bottom())),
            );
        }
        self.base.invalidate();
        
        old
    }
    fn remove_child_from(&mut self, _: &mut MemberBase, index: usize) -> Option<Box<dyn controls::Control>> {
        if index < self.children.len() {
            let item = self.children.remove(index);
            let widget = common::cast_control_to_gtkwidget(item.as_ref());
            Object::from(self.base.widget.clone()).downcast::<GtkBox>().unwrap().remove::<Widget>(&Object::from(widget).downcast().unwrap());
            self.base.invalidate();
            
            Some(item)
        } else {
            None
        }
    }
    fn child_at(&self, index: usize) -> Option<&dyn controls::Control> {
        self.children.get(index).map(|m| m.as_ref())
    }
    fn child_at_mut(&mut self, index: usize) -> Option<&mut dyn controls::Control> {
        //self.children.get_mut(index).map(|c| c.as_mut()) //the anonymous lifetime #1 does not necessarily outlive the static lifetime
        if let Some(c) = self.children.get_mut(index) {
            Some(c.as_mut())
        } else {
            None
        }
    }
}

#[allow(dead_code)]
pub(crate) fn spawn() -> Box<dyn controls::Control> {
    LinearLayout::with_orientation(layout::Orientation::Vertical).into_control()
}

fn on_size_allocate(this: &::gtk::Widget, _allo: &::gtk::Rectangle) {
    let mut ll = this.clone().upcast::<Widget>();
    let ll = common::cast_gtk_widget_to_member_mut::<LinearLayout>(&mut ll).unwrap();

    let measured_size = ll.as_inner_mut().base().measured;
    ll.call_on_size(measured_size.0 as u16, measured_size.1 as u16);
}

default_impls_as!(LinearLayout);
