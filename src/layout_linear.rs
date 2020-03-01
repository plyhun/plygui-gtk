use crate::common::{self, *};

use gtk::{Box as GtkBox, BoxExt, Cast, ContainerExt, OrientableExt, Widget, WidgetExt};

pub type LinearLayout = AMember<AControl<AContainer<AMultiContainer<ALinearLayout<GtkLinearLayout>>>>>;

#[repr(C)]
pub struct GtkLinearLayout {
    base: common::GtkControlBase<LinearLayout>,
    children: Vec<Box<dyn controls::Control>>,
}
impl<O: controls::LinearLayout> NewLinearLayoutInner<O> for GtkLinearLayout {
    fn with_uninit_params(ptr: &mut mem::MaybeUninit<O>, orientation: layout::Orientation) -> Self {
        let ptr = ptr as *mut _ as *mut c_void;
        let ll = reckless::RecklessBox::new();
        let ll = ll.upcast::<GtkBox>();
        ll.set_orientation(common::orientation_to_gtk(orientation));
        let ll = ll.upcast::<Widget>();
        ll.connect_size_allocate(on_size_allocate::<O>);
        let mut ll = GtkLinearLayout {
            base: common::GtkControlBase::with_gtk_widget(ll),
            children: Vec::new(),
        };
        ll.base.set_pointer(ptr);
        ll
    }
}
impl LinearLayoutInner for GtkLinearLayout {
    fn with_orientation(orientation: layout::Orientation) -> Box<dyn controls::LinearLayout> {
        let mut b: Box<mem::MaybeUninit<LinearLayout>> = Box::new_uninit();
        let ab = AMember::with_inner(
            AControl::with_inner(
                AContainer::with_inner(
                    AMultiContainer::with_inner(
                        ALinearLayout::with_inner(
                            <Self as NewLinearLayoutInner<LinearLayout>>::with_uninit_params(b.as_mut(), orientation),
                        )
                    ),
                )
            ),
        );
        unsafe {
	        b.as_mut_ptr().write(ab);
	        b.assume_init()
        }
    }
}

impl HasNativeIdInner for GtkLinearLayout {
    type Id = common::GtkWidget;

    fn native_id(&self) -> Self::Id {
        self.base.widget.clone().into()
    }
}

impl HasSizeInner for GtkLinearLayout {
    fn on_size_set(&mut self, _: &mut MemberBase, _: (u16, u16)) -> bool {
        self.base.invalidate();
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
    fn measure(&mut self, member: &mut MemberBase, control: &mut ControlBase, parent_width: u16, parent_height: u16) -> (u16, u16, bool) {
        let orientation = self.orientation(member);
        let old_size = control.measured;
        
        control.measured = match control.visibility {
            types::Visibility::Gone => (0, 0),
            _ => {
                let mut w = 0;
                let mut h = 0;
                for child in self.children.as_mut_slice() {
                    match orientation {
                        layout::Orientation::Horizontal => {
                            let (cw, ch, _) = child.measure(cmp::max(0, parent_width as i32 - w as i32) as u16, cmp::max(0, parent_height as i32) as u16);
                            w += cw;
                            h = cmp::max(h, ch);
                        }
                        layout::Orientation::Vertical => {
                            let (cw, ch, _) = child.measure(cmp::max(0, parent_width as i32) as u16, cmp::max(0, parent_height as i32 - h as i32) as u16);
                            w = cmp::max(w, cw);
                            h += ch;
                        }
                    }
                }
                let w = match control.layout.width {
                    layout::Size::Exact(w) => w,
                    layout::Size::MatchParent => parent_width,
                    layout::Size::WrapContent => {
                        cmp::max(0, w as i32) as u16
                    }
                };
                let h = match control.layout.height {
                    layout::Size::Exact(h) => h,
                    layout::Size::MatchParent => parent_height,
                    layout::Size::WrapContent => {
                        cmp::max(0, h as i32) as u16
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
        let o = self.orientation(member);
	    let (lm, tm, rm, bm) = self.base.margins().into();
        let self2 = self.base.as_control();
        let mut x = 0;
	    let mut y = 0;
	    for ref mut child in self.children.as_mut_slice() {
            match o {
	            layout::Orientation::Horizontal => {
	            	child.on_added_to_container(self2, 0, 0, utils::coord_to_size(cmp::max(0, pw as i32 - x - lm - rm)), utils::coord_to_size(cmp::max(0, ph as i32 - tm - bm)));
	            	let (cw, _) = child.size();
	                x += cw as i32;
	            },
	            layout::Orientation::Vertical => {
	                child.on_added_to_container(self2, 0, 0, utils::coord_to_size(cmp::max(0, pw as i32 - lm - rm)), utils::coord_to_size(cmp::max(0, ph as i32 - y - tm - bm)));
	                let (_, ch) = child.size();
	                y += ch as i32;
	            },
	        }
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
    fn orientation(&self, _: &MemberBase) -> layout::Orientation {
        let gtk_self = Object::from(self.base.widget.clone()).downcast::<GtkBox>().unwrap();
        common::gtk_to_orientation(gtk_self.get_orientation())
    }
    fn set_orientation(&mut self, _: &mut MemberBase, orientation: layout::Orientation) {
        let gtk_self = Object::from(self.base.widget.clone()).downcast::<GtkBox>().unwrap();
        gtk_self.set_orientation(common::orientation_to_gtk(orientation));
        self.base.invalidate();
    }
}

impl ContainerInner for GtkLinearLayout {
    fn find_control_mut(&mut self, arg: types::FindBy) -> Option<&mut dyn controls::Control> {
        for child in self.children.as_mut_slice() {
            match arg {
                types::FindBy::Id(ref id) => {
                    if child.as_member_mut().id() == *id {
                        return Some(child.as_mut());
                    }
                }
                types::FindBy::Tag(ref tag) => {
                    if let Some(mytag) = child.as_member_mut().tag() {
                        if tag.as_str() == mytag {
                            return Some(child.as_mut());
                        }
                    }
                }
            }
            if let Some(c) = child.is_container_mut() {
                let ret = c.find_control_mut(arg.clone());
                if ret.is_none() {
                    continue;
                }
                return ret;
            }
        }
        None
    }
    fn find_control(&self, arg: types::FindBy) -> Option<&dyn controls::Control> {
        for child in self.children.as_slice() {
            match arg {
                types::FindBy::Id(ref id) => {
                    if child.as_member().id() == *id {
                        return Some(child.as_ref());
                    }
                }
                types::FindBy::Tag(ref tag) => {
                    if let Some(mytag) = child.as_member().tag() {
                        if tag.as_str() == mytag {
                            return Some(child.as_ref());
                        }
                    }
                }
            }
            if let Some(c) = child.is_container() {
                let ret = c.find_control(arg.clone());
                if ret.is_none() {
                    continue;
                }
                return ret;
            }
        }
        None
    }}

impl MultiContainerInner for GtkLinearLayout {
    fn len(&self) -> usize {
        self.children.len()
    }
    fn set_child_to(&mut self, base: &mut MemberBase, index: usize, child: Box<dyn controls::Control>) -> Option<Box<dyn controls::Control>> {
        let self2 = unsafe { utils::base_to_impl_mut::<LinearLayout>(base) };
        let boxc = Object::from(self.base.widget.clone()).downcast::<GtkBox>().unwrap();
        
        self.children.insert(index, child);
        let old = if (index + 1) < self.children.len() {
            let mut old = self.children.remove(index + 1);
            let widget = common::cast_control_to_gtkwidget(old.as_mut());
            let widget = Object::from(widget).downcast::<Widget>().unwrap();
            boxc.remove(&widget);
            if self2.inner().base.coords.is_some() {
                old.on_removed_from_container(self2);
            }
            Some(old)
        } else {
            None
        };

        let widget = common::cast_control_to_gtkwidget(self.children.get_mut(index).unwrap().as_mut());
        let widget = Object::from(widget).downcast::<Widget>().unwrap();
        boxc.add(&widget);
        boxc.set_child_position(&widget, index as i32);
        if self2.inner().base.coords.is_some() {
            let (pw, ph) = self2.inner().base.measured;
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
impl Spawnable for GtkLinearLayout {
    fn spawn() -> Box<dyn controls::Control> {
        Self::with_orientation(layout::Orientation::Vertical).into_control()
    }
}

fn on_size_allocate<O: controls::LinearLayout>(this: &::gtk::Widget, _allo: &::gtk::Rectangle) {
    let mut ll = this.clone().upcast::<Widget>();
    let ll = common::cast_gtk_widget_to_member_mut::<LinearLayout>(&mut ll).unwrap();

    let measured_size = ll.inner_mut().base.measured;
    ll.call_on_size::<O>(measured_size.0 as u16, measured_size.1 as u16);
    
    let mut x = 0;
    let mut y = 0;
    let o = controls::HasOrientation::orientation(ll);
    let ll = ll.inner_mut().inner_mut().inner_mut().inner_mut().inner_mut();
    for i in 0..ll.children.len() {
        let item = &mut ll.children[i];
        match o {
            layout::Orientation::Horizontal => {
                let (cw, _, _) = item.measure(cmp::max(0, measured_size.0 as i32 - x) as u16, cmp::max(0, measured_size.1 as i32) as u16);
                item.draw(Some((x, 0)));
                x += cw as i32;
            },
            layout::Orientation::Vertical => {
                let (_, ch, _) = item.measure(cmp::max(0, measured_size.0 as i32) as u16, cmp::max(0, measured_size.1 as i32 - y) as u16);
                item.draw(Some((0, y)));
                y += ch as i32;
            },
        }
    }
}
