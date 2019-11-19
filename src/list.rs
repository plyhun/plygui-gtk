use crate::common::{self, *};

use gtk::{ListBox, ListBoxExt, ContainerExt, ListBoxRow, ListBoxRowExt, ScrolledWindow, ScrolledWindowExt, PolicyType};

pub type List = Member<Control<Adapter<GtkList>>>;

#[repr(C)]
pub struct GtkList {
    base: GtkControlBase<List>,
    boxc: ListBox,
    items: Vec<Box<dyn controls::Control>>,
}

impl GtkList {
    fn add_item_inner(&mut self, base: &mut MemberBase, i: usize, y: &mut i32) {
        let (member, control, adapter) = List::adapter_base_parts_mut(base);
        let (pw, ph) = control.measured;
        let this: &mut List = unsafe { utils::base_to_impl_mut(member) };
        
        let mut item = adapter.adapter.spawn_item_view(i, this);
        item.on_added_to_container(this, 0, *y, utils::coord_to_size(pw as i32) as u16, utils::coord_to_size(ph as i32) as u16);
        let widget = common::cast_control_to_gtkwidget(item.as_mut());
                
        let (_, yy) = item.size();
        self.items.push(item);
        *y += yy as i32;
        
        this.as_inner_mut().as_inner_mut().as_inner_mut().boxc.insert(&Object::from(widget).downcast::<Widget>().unwrap(), i as i32);
    }
    fn remove_item_inner(&mut self, base: &mut MemberBase, i: usize) {
        let this: &mut List = unsafe { utils::base_to_impl_mut(base) };
        self.items.remove(i).on_removed_from_container(this); 
        let row = this.as_inner_mut().as_inner_mut().as_inner_mut().boxc.get_row_at_index(i as i32).unwrap();
        
        this.as_inner_mut().as_inner_mut().as_inner_mut().boxc.remove(&row);
    }
}

impl AdapterViewInner for GtkList {
	fn with_adapter(adapter: Box<dyn types::Adapter>) -> Box<Member<Control<Adapter<Self>>>> {
		let mut li = Box::new(Member::with_inner(
            Control::with_inner(
                Adapter::with_inner(
	                GtkList {
	                    base: common::GtkControlBase::with_gtk_widget(reckless::RecklessScrolledWindow::new().upcast::<Widget>()),
	                    boxc: ListBox::new(),
	                    items: Vec::new(),
	                },
	                adapter
                ),
                (),
            ),
            MemberFunctions::new(_as_any, _as_any_mut, _as_member, _as_member_mut),
        ));
        {
            let ptr = li.as_ref() as *const _ as *mut std::os::raw::c_void;
            li.as_inner_mut().as_inner_mut().as_inner_mut().base.set_pointer(ptr);
            let mut boxc = li.as_inner_mut().as_inner_mut().as_inner_mut().boxc.clone().upcast::<Object>();
            common::set_pointer(&mut boxc, ptr);
        }
        {
            let self_widget: Object = Object::from(li.as_inner_mut().as_inner_mut().as_inner_mut().base.widget.clone()).into();
            let scr = self_widget.downcast::<ScrolledWindow>().unwrap();
            scr.set_policy(PolicyType::Never, PolicyType::Always);
            li.as_inner_mut().as_inner_mut().as_inner_mut().boxc.connect_row_activated(on_activated);
            scr.add(&li.as_inner_mut().as_inner_mut().as_inner_mut().boxc);
        }
        li.as_inner_mut().as_inner_mut().as_inner_mut().base.widget().connect_size_allocate(on_size_allocate);
        li
	}
    fn on_item_change(&mut self, base: &mut MemberBase, value: types::Change) {
        let mut y = 0;
        {
            for item in self.items.as_slice() {
                let (_, yy) = item.size();
                y += yy as i32;
            }
        }
        match value {
            types::Change::Added(at) => {
                self.add_item_inner(base, at, &mut y);
            },
            types::Change::Removed(at) => {
                self.remove_item_inner(base, at);
            },
            types::Change::Edited(_) => {
            },
        }
    }
}
impl ContainerInner for GtkList {
    fn find_control_mut(&mut self, arg: types::FindBy) -> Option<&mut dyn controls::Control> {
        for child in self.items.as_mut_slice() {
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
        for child in self.items.as_slice() {
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
    }
}

impl HasLayoutInner for GtkList {
    fn on_layout_changed(&mut self, _base: &mut MemberBase) {
        self.base.invalidate();
    }
}

impl ControlInner for GtkList {
    fn on_added_to_container(&mut self, member: &mut MemberBase, control: &mut ControlBase, _parent: &dyn controls::Container, x: i32, y: i32, pw: u16, ph: u16) {
        self.measure(member, control, pw, ph);
        control.coords = Some((x, y));
        self.draw(member, control);
        let (member, _, adapter) = List::adapter_base_parts_mut(member);

        let mut y = 0;
        for i in 0..adapter.adapter.len() {
            self.add_item_inner(member, i, &mut y);
        }
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
        use plygui_api::markup::MEMBER_TYPE_TEXT;
        fill_from_markup_base!(self, base, markup, registry, List, [MEMBER_TYPE_TEXT]);
        fill_from_markup_label!(self, &mut base.member, markup);
    }
}

impl HasNativeIdInner for GtkList {
    type Id = common::GtkWidget;

    unsafe fn native_id(&self) -> Self::Id {
        self.base.widget.clone().into()
    }
}

impl HasSizeInner for GtkList {
    fn on_size_set(&mut self, base: &mut MemberBase, (width, height): (u16, u16)) -> bool {
        use plygui_api::controls::HasLayout;

        let this = base.as_any_mut().downcast_mut::<List>().unwrap();
        this.set_layout_width(layout::Size::Exact(width));
        this.set_layout_width(layout::Size::Exact(height));
        self.base.widget().set_size_request(width as i32, height as i32);
        true
    }
}

impl HasVisibilityInner for GtkList {
    fn on_visibility_set(&mut self, _: &mut MemberBase, _: types::Visibility) -> bool {
        self.base.invalidate()
    }
}

impl MemberInner for GtkList {}

impl Drawable for GtkList {
    fn draw(&mut self, _: &mut MemberBase, control: &mut ControlBase) {
        self.base.draw(control);
        self.boxc.show();
    }
    fn measure(&mut self, _: &mut MemberBase, control: &mut ControlBase, parent_width: u16, parent_height: u16) -> (u16, u16, bool) {
        let old_size = control.measured;
        control.measured = match control.visibility {
            types::Visibility::Gone => (0, 0),
            _ => {
                let w = match control.layout.width {
                    layout::Size::MatchParent => parent_width,
                    layout::Size::Exact(w) => w,
                    layout::Size::WrapContent => defaults::THE_ULTIMATE_ANSWER_TO_EVERYTHING,
                };
                let h = match control.layout.height {
                    layout::Size::MatchParent => parent_height,
                    layout::Size::Exact(h) => h,
                    layout::Size::WrapContent => defaults::THE_ULTIMATE_ANSWER_TO_EVERYTHING,
                };
                (cmp::max(0, w as i32) as u16, cmp::max(0, h as i32) as u16)
            }
        };
        (control.measured.0, control.measured.1, control.measured != old_size)
    }
    fn invalidate(&mut self, _: &mut MemberBase, _: &mut ControlBase) {
        self.base.invalidate();
    }
}

fn on_size_allocate(this: &::gtk::Widget, _allo: &::gtk::Rectangle) {
    let mut ll = this.clone().upcast::<Widget>();
    let ll = common::cast_gtk_widget_to_member_mut::<List>(&mut ll).unwrap();

    let measured_size = ll.as_inner().base().measured;
    ll.call_on_size(measured_size.0 as u16, measured_size.1 as u16);
    
    println!("{:?}", measured_size);
    
    let mut y = 0;
    let list = ll.as_inner_mut().as_inner_mut().as_inner_mut();
    for i in 0..list.items.len() {
        let item = &mut list.items[i];
        let (_, ch, _) = item.measure(cmp::max(0, measured_size.0 as i32) as u16, cmp::max(0, measured_size.1 as i32) as u16);
        item.draw(Some((0, y)));
        y += ch as i32;
    }
}
fn on_activated(this: &ListBox, row: &ListBoxRow) {
    let i = row.get_index();
    if i < 0 {
        return;
    }
    let mut ll = this.clone().upcast::<Widget>();
    let ll = common::cast_gtk_widget_to_member_mut::<List>(&mut ll).unwrap();
    let mut ll2 = this.clone().upcast::<Widget>();
    let ll2 = common::cast_gtk_widget_to_member_mut::<List>(&mut ll2).unwrap();
    let item_view = ll.as_inner_mut().as_inner_mut().as_inner_mut().items.get_mut(i as usize).unwrap();
    if let Some(ref mut callback) = ll2.as_inner_mut().as_inner_mut().base_mut().on_item_click {
        let mut ll2 = this.clone().upcast::<Widget>();
        let ll2 = common::cast_gtk_widget_to_member_mut::<List>(&mut ll2).unwrap();
        (callback.as_mut())(ll2, i as usize, item_view.as_mut());
    }
}

default_impls_as!(List);
