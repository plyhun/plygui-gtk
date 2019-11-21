use crate::common::{self, *};

use gtk::{Cast, OrientableExt, Paned, PanedExt, Widget, WidgetExt};

pub type Splitted = Member<Control<MultiContainer<GtkSplitted>>>;

#[repr(C)]
pub struct GtkSplitted {
    base: common::GtkControlBase<Splitted>,
    splitter: f32,
    first: Box<dyn controls::Control>,
    second: Box<dyn controls::Control>,
}

impl GtkSplitted {
    fn update_splitter(&mut self, control: &ControlBase) {
        let self_widget = self.base.widget();
        let orientation = self.layout_orientation();
        
        println!("{:?}", control.measured);
        match orientation {
            layout::Orientation::Horizontal => self_widget.downcast::<Paned>().unwrap().set_position((control.measured.0 as f32 * self.splitter) as i32),
            layout::Orientation::Vertical => self_widget.downcast::<Paned>().unwrap().set_position((control.measured.1 as f32 * self.splitter) as i32),
        }
    }
    fn children_sizes(&self, control: &ControlBase) -> (u16, u16) {
        let (w, h) = control.measured;
        let self_widget = self.base.widget();
        let o = self.layout_orientation();
        let handle = 6; // no access to handle-size
        let (target, start, end) = match o {
            layout::Orientation::Horizontal => (w, self_widget.get_margin_start(), self_widget.get_margin_end()),
            layout::Orientation::Vertical => (h, self_widget.get_margin_top(), self_widget.get_margin_bottom()),
        };
        (
            utils::coord_to_size((target as f32 * self.splitter) as i32 - start - (handle / 2)),
            utils::coord_to_size((target as f32 * (1.0 - self.splitter)) as i32 - end - (handle / 2)),
        )
    }
    fn update_children_layout(&mut self, base: &ControlBase) -> (u16, u16) {
        let orientation = self.layout_orientation();
        let (first_size, second_size) = self.children_sizes(base);
        let (width, height) = base.measured;
        let (lm, tm, rm, bm) = self.base.margins().into();
        let mut w = 0;
        let mut h = 0;
        for (size, child) in [(first_size, self.first.as_mut()), (second_size, self.second.as_mut())].iter_mut() {
            match orientation {
                layout::Orientation::Horizontal => {
                    let (cw, ch, _) = child.measure(cmp::max(0, *size) as u16, cmp::max(0, height as i32 - tm - bm) as u16);
                    w += cw;
                    h = cmp::max(h, ch);
                }
                layout::Orientation::Vertical => {
                    let (cw, ch, _) = child.measure(cmp::max(0, width as i32 - lm - rm) as u16, cmp::max(0, *size) as u16);
                    w = cmp::max(w, cw);
                    h += ch;
                }
            }
        }
        (w, h)
    }
}

impl SplittedInner for GtkSplitted {
    fn with_content(first: Box<dyn controls::Control>, second: Box<dyn controls::Control>, orientation: layout::Orientation) -> Box<Splitted> {
        let mut ll = Box::new(Member::with_inner(
            Control::with_inner(
                MultiContainer::with_inner(
                    GtkSplitted {
                        base: common::GtkControlBase::with_gtk_widget(reckless::RecklessPaned::new().upcast::<Widget>()),
                        first: first,
                        splitter: 0.5,
                        second: second,
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
            let paned = Object::from(ll.as_inner_mut().as_inner_mut().as_inner_mut().base.widget.clone()).downcast::<Paned>().unwrap();
            paned.set_orientation(common::orientation_to_gtk(orientation));
            paned.pack1(&Object::from(common::cast_control_to_gtkwidget(ll.as_inner_mut().as_inner_mut().as_inner_mut().first())).downcast::<Widget>().unwrap(), false, true);
            paned.pack2(&Object::from(common::cast_control_to_gtkwidget(ll.as_inner_mut().as_inner_mut().as_inner_mut().second())).downcast::<Widget>().unwrap(), false, true);
            paned.connect_property_position_notify(on_property_position_notify);
        }
        {
            let mut self_widget = ll.as_inner_mut().as_inner_mut().as_inner_mut().base.widget();
            self_widget.connect_size_allocate(on_size_allocate);
            ll.as_inner_mut()
                .as_inner_mut()
                .as_inner_mut()
                .update_splitter(common::cast_gtk_widget_to_member::<Splitted>(&mut self_widget).unwrap().as_inner().base());
        }
        ll
    }
    fn set_splitter(&mut self, base: &mut MemberBase, pos: f32) {
        let pos = pos % 1.0;
        self.splitter = pos;
        let (_, control) = Splitted::control_base_parts_mut(base);
        self.update_splitter(control);
    }
    fn splitter(&self) -> f32 {
        self.splitter
    }

    fn first(&self) -> &dyn controls::Control {
        self.first.as_ref()
    }
    fn second(&self) -> &dyn controls::Control {
        self.second.as_ref()
    }
    fn first_mut(&mut self) -> &mut dyn controls::Control {
        self.first.as_mut()
    }
    fn second_mut(&mut self) -> &mut dyn controls::Control {
        self.second.as_mut()
    }
}

impl HasNativeIdInner for GtkSplitted {
    type Id = common::GtkWidget;

    unsafe fn native_id(&self) -> Self::Id {
        self.base.widget.clone().into()
    }
}

impl HasSizeInner for GtkSplitted {
    fn on_size_set(&mut self, _: &mut MemberBase, (width, height): (u16, u16)) -> bool {
        self.base.widget().set_size_request(width as i32, height as i32);
        true
    }
}

impl HasVisibilityInner for GtkSplitted {
    fn on_visibility_set(&mut self, _: &mut MemberBase, _: types::Visibility) -> bool {
        self.base.invalidate()
    }
}

impl MemberInner for GtkSplitted {}

impl Drawable for GtkSplitted {
    fn draw(&mut self, _: &mut MemberBase, control: &mut ControlBase) {
        self.base.draw(control);
        self.first.draw(Some((0, 0)));
        self.second.draw(Some((0, 0)));
    }
    fn measure(&mut self, _: &mut MemberBase, control: &mut ControlBase, parent_width: u16, parent_height: u16) -> (u16, u16, bool) {
        let old_size = control.measured;
        control.measured = match control.visibility {
            types::Visibility::Gone => (0, 0),
            _ => {
                let (w, h) = self.update_children_layout(control);
                let w = match control.layout.width {
                    layout::Size::Exact(w) => w,
                    layout::Size::MatchParent => parent_width,
                    layout::Size::WrapContent => cmp::max(0, w as i32) as u16
                };
                let h = match control.layout.height {
                    layout::Size::Exact(h) => h,
                    layout::Size::MatchParent => parent_height,
                    layout::Size::WrapContent => cmp::max(0, h as i32) as u16
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

impl HasLayoutInner for GtkSplitted {
    fn on_layout_changed(&mut self, base: &mut MemberBase) {
        let control = unsafe { utils::base_to_impl_mut::<Splitted>(base).as_inner_mut().base_mut() };
        self.update_splitter(control);
        self.base.invalidate();
    }
}

impl ControlInner for GtkSplitted {
    fn on_added_to_container(&mut self, member: &mut MemberBase, control: &mut ControlBase, _parent: &dyn controls::Container, x: i32, y: i32, pw: u16, ph: u16) {
        control.measured = (pw, ph); // for update_splitter only
        self.update_splitter(control);
        self.measure(member, control, pw, ph);
        control.coords = Some((x, y));
        self.draw(member, control);

        let (first, second) = self.children_sizes(control);
        let o = self.layout_orientation();
        let (lm, tm, rm, bm) = self.base.margins().into();
        let self2 = self.base.as_control();

        match o {
            layout::Orientation::Horizontal => {
                let h = utils::coord_to_size(ph as i32 - tm - bm);
                self.first.on_added_to_container(self2, 0, 0, first, h);
                self.second.on_added_to_container(self2, 0, 0, second, h);
            }
            layout::Orientation::Vertical => {
                let w = utils::coord_to_size(pw as i32 - lm - rm);
                self.first.on_added_to_container(self2, 0, 0, w, first);
                self.second.on_added_to_container(self2, 0, 0, w, second);
            }
        }
    }
    fn on_removed_from_container(&mut self, _: &mut MemberBase, _: &mut ControlBase, _: &dyn controls::Container) {
        let self2 = self.base.as_control();
        for child in [self.first.as_mut(), self.second.as_mut()].iter_mut() {
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
        use plygui_api::markup::MEMBER_TYPE_SPLITTED;

        fill_from_markup_base!(self, markup, registry, Splitted, [MEMBER_TYPE_SPLITTED]);
        fill_from_markup_children!(self, markup, registry);
    }
}

impl HasOrientationInner for GtkSplitted {
    fn layout_orientation(&self) -> layout::Orientation {
        let gtk_self = Object::from(self.base.widget.clone()).downcast::<Paned>().unwrap();
        common::gtk_to_orientation(gtk_self.get_orientation())
    }
    fn set_layout_orientation(&mut self, _: &mut MemberBase, orientation: layout::Orientation) {
        let gtk_self = Object::from(self.base.widget.clone()).downcast::<Paned>().unwrap();
        gtk_self.set_orientation(common::orientation_to_gtk(orientation));
        self.base.invalidate();
    }
}

impl ContainerInner for GtkSplitted {
    fn find_control_mut(&mut self, arg: types::FindBy) -> Option<&mut dyn controls::Control> {
        match arg {
            types::FindBy::Id(id) => {
                if self.first().as_member().id() == id {
                    return Some(self.first_mut());
                }
                if self.second().as_member().id() == id {
                    return Some(self.second_mut());
                }
            }
            types::FindBy::Tag(ref tag) => {
                if let Some(mytag) = self.first.as_member().tag() {
                    if tag.as_str() == mytag {
                        return Some(self.first_mut());
                    }
                }
                if let Some(mytag) = self.second.as_member().tag() {
                    if tag.as_str() == mytag {
                        return Some(self.second_mut());
                    }
                }
            }
        }

        let self2: &mut GtkSplitted = unsafe { mem::transmute(self as *mut GtkSplitted) }; // bck is stupid
        if let Some(c) = self.first_mut().is_container_mut() {
            let ret = c.find_control_mut(arg.clone());
            if ret.is_some() {
                return ret;
            }
        }
        if let Some(c) = self2.second_mut().is_container_mut() {
            let ret = c.find_control_mut(arg);
            if ret.is_some() {
                return ret;
            }
        }
        None
    }
    fn find_control(&self, arg: types::FindBy) -> Option<&dyn controls::Control> {
        match arg {
            types::FindBy::Id(id) => {
                if self.first().as_member().id() == id {
                    return Some(self.first());
                }
                if self.second().as_member().id() == id {
                    return Some(self.second());
                }
            }
            types::FindBy::Tag(ref tag) => {
                if let Some(mytag) = self.first.as_member().tag() {
                    if tag.as_str() == mytag {
                        return Some(self.first.as_ref());
                    }
                }
                if let Some(mytag) = self.second.as_member().tag() {
                    if tag.as_str() == mytag {
                        return Some(self.second.as_ref());
                    }
                }
            }
        }
        if let Some(c) = self.first().is_container() {
            let ret = c.find_control(arg.clone());
            if ret.is_some() {
                return ret;
            }
        }
        if let Some(c) = self.second().is_container() {
            let ret = c.find_control(arg);
            if ret.is_some() {
                return ret;
            }
        }
        None
    }
}

impl MultiContainerInner for GtkSplitted {
    fn len(&self) -> usize {
        2
    }
    fn set_child_to(&mut self, _: &mut MemberBase, index: usize, mut child: Box<dyn controls::Control>) -> Option<Box<dyn controls::Control>> {
        let self2 = self.base.as_control();

        let (pw, ph) = self2.as_inner().base().measured;
        let orientation = self.layout_orientation();
        let (first, second) = self.children_sizes(self2.as_inner().base());
        let (lm, tm, rm, bm) = self.base.margins().into();
        let gtk_self = Object::from(self.base.widget.clone()).downcast::<Paned>().unwrap();
        {
            match index {
                0 => {
                    mem::swap(&mut self.first, &mut child);

                    let widget = common::cast_control_to_gtkwidget(self.first.as_mut());
                    gtk_self.add1(&Object::from(widget).downcast::<Widget>().unwrap());
                    child.on_removed_from_container(self2);
                    match orientation {
                        layout::Orientation::Horizontal => {
                            self.first.on_added_to_container(self2, 0, 0, first, utils::coord_to_size(ph as i32 - tm - bm));
                        }
                        layout::Orientation::Vertical => {
                            self.first.on_added_to_container(self2, 0, 0, utils::coord_to_size(pw as i32 - lm - rm), first);
                        }
                    }
                }
                1 => {
                    mem::swap(&mut self.second, &mut child);

                    let widget = common::cast_control_to_gtkwidget(self.first.as_mut());
                    gtk_self.downcast::<Paned>().unwrap().add2(&Object::from(widget).downcast::<Widget>().unwrap());
                    child.on_removed_from_container(self2);
                    match orientation {
                        layout::Orientation::Horizontal => {
                            self.second.on_added_to_container(self2, 0, 0, second, utils::coord_to_size(ph as i32 - tm - bm));
                        }
                        layout::Orientation::Vertical => {
                            self.second.on_added_to_container(self2, 0, 0, utils::coord_to_size(pw as i32 - lm - rm), second);
                        }
                    }
                }
                _ => return None,
            }
        }
        self.base.invalidate();
        Some(child)
    }
    fn remove_child_from(&mut self, _: &mut MemberBase, _: usize) -> Option<Box<dyn controls::Control>> {
        None
    }
    fn child_at(&self, index: usize) -> Option<&dyn controls::Control> {
        match index {
            0 => Some(self.first()),
            1 => Some(self.second()),
            _ => None,
        }
    }
    fn child_at_mut(&mut self, index: usize) -> Option<&mut dyn controls::Control> {
        match index {
            0 => Some(self.first_mut()),
            1 => Some(self.second_mut()),
            _ => None,
        }
    }
}

/*#[allow(dead_code)]
pub(crate) fn spawn() -> Box<controls::Control> {
    Splitted::with_orientation(layout::Orientation::Vertical).into_control()
}*/

fn on_size_allocate(this: &::gtk::Widget, _: &::gtk::Rectangle) {
    let mut ll = this.clone().upcast::<Widget>();
    let ll = common::cast_gtk_widget_to_member_mut::<Splitted>(&mut ll).unwrap();

    let measured_size = ll.as_inner().base().measured;
    ll.call_on_size(measured_size.0 as u16, measured_size.1 as u16);
}
fn on_property_position_notify(this: &::gtk::Paned) {
    use plygui_api::controls::{HasOrientation, HasSize};

    let position = this.get_position();
    if position < 1 {
        return;
    }

    let mut ll = this.clone().upcast::<Widget>();
    let ll = common::cast_gtk_widget_to_member_mut::<Splitted>(&mut ll).unwrap();
    let orientation = ll.layout_orientation();
    let (width, height) = ll.size();
    println!("splitted {}/{}", width, height);
    let splitter = position as f32
        / match orientation {
            layout::Orientation::Vertical => {
                if height > 0 {
                    height as f32
                } else {
                    position as f32 * 2.0
                }
            }
            layout::Orientation::Horizontal => {
                if width > 0 {
                    width as f32
                } else {
                    position as f32 * 2.0
                }
            }
        };
    let member = unsafe { &mut *(ll.base_mut() as *mut MemberBase) };
    let control = unsafe { &mut *(ll.as_inner_mut().base_mut() as *mut ControlBase) };
    let ll = ll.as_inner_mut().as_inner_mut().as_inner_mut();
    ll.splitter = splitter;
    ll.measure(member, control, width, height);
    ll.first.draw(Some((0,0)));
    ll.second.draw(Some((0,0)));
}

default_impls_as!(Splitted);
