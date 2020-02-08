use crate::common::{self, *};

use gtk::{ListBox, ListBoxExt, ContainerExt, ListBoxRow, ListBoxRowExt, ScrolledWindow, ScrolledWindowExt, PolicyType};

pub type List = AMember<AControl<AContainer<AAdapted<AList<GtkList>>>>>;

#[repr(C)]
pub struct GtkList {
    base: GtkControlBase<List>,
    boxc: ListBox,
    items: Vec<Box<dyn controls::Control>>,
    h_left_clicked: Option<callbacks::OnItemClick>,
}

impl GtkList {
    fn add_item_inner(&mut self, base: &mut MemberBase, i: usize, y: &mut i32) {
        let (member, control, adapter, _) = unsafe { List::adapter_base_parts_mut(base) };
        let (pw, ph) = control.measured;
        let this: &mut List = unsafe { utils::base_to_impl_mut(member) };
        
        let mut item = adapter.adapter.spawn_item_view(i, this);
        item.on_added_to_container(this, 0, *y, utils::coord_to_size(pw as i32) as u16, utils::coord_to_size(ph as i32) as u16);
        let widget = common::cast_control_to_gtkwidget(item.as_mut());
                
        let (_, yy) = item.size();
        self.items.insert(i, item);
        *y += yy as i32;
        
        this.inner_mut().inner_mut().inner_mut().inner_mut().inner_mut().boxc.insert(&Object::from(widget).downcast::<Widget>().unwrap(), i as i32);
    }
    fn remove_item_inner(&mut self, base: &mut MemberBase, i: usize) {
        let this: &mut List = unsafe { utils::base_to_impl_mut(base) };
        self.items.remove(i).on_removed_from_container(this); 
        let row = this.inner_mut().inner_mut().inner_mut().inner_mut().inner_mut().boxc.get_row_at_index(i as i32).unwrap();
        
        this.inner_mut().inner_mut().inner_mut().inner_mut().inner_mut().boxc.remove(&row);
    }
}
impl<O: controls::List> NewListInner<O> for GtkList {
    fn with_uninit(ptr: &mut mem::MaybeUninit<O>) -> Self {
        let ptr = ptr as *mut _ as *mut c_void;
        let li = reckless::RecklessScrolledWindow::new();
        let li = li.upcast::<Widget>();
        li.connect_size_allocate(on_size_allocate::<O>);
        let mut li = GtkList {
            base: common::GtkControlBase::with_gtk_widget(li),
            boxc: ListBox::new(),
            items: Vec::new(),
            h_left_clicked: None,
        };
        li.boxc.connect_row_activated(on_activated::<O>);
        let scr = Object::from(li.base.widget.clone()).downcast::<ScrolledWindow>().unwrap();
        scr.set_policy(PolicyType::Never, PolicyType::Always);
        scr.add(&li.boxc);
        common::set_pointer(&mut li.boxc.clone().upcast(), ptr);
        li.base.set_pointer(ptr);  
        li
    }
}
impl ListInner for GtkList {
	fn with_adapter(adapter: Box<dyn types::Adapter>) -> Box<dyn controls::List> {
		let len = adapter.len();
        let mut b: Box<mem::MaybeUninit<List>> = Box::new_uninit();
        let mut ab = AMember::with_inner(
            AControl::with_inner(
                AContainer::with_inner(
                    AAdapted::with_inner(
                        AList::with_inner(
                            <Self as NewListInner<List>>::with_uninit(b.as_mut())
                        ),
                        adapter,
                        &mut b,
                    ),
                )
            ),
        );
        ab.inner_mut().inner_mut().inner_mut().inner_mut().inner_mut().items = Vec::with_capacity(len);
        let mut bb = unsafe {
	        b.as_mut_ptr().write(ab);
	        b.assume_init()
        };
        let (member, _, adapter, list) = unsafe { List::adapter_base_parts_mut(&mut bb.base) };

		let mut y = 0;
        for i in 0..adapter.adapter.len() {
            list.inner_mut().add_item_inner(member, i, &mut y);
        }
        bb
	}
}
impl ItemClickableInner for GtkList {
    fn item_click(&mut self, i: usize, item_view: &mut dyn controls::Control, _skip_callbacks: bool) {
        let mut this = Object::from(self.base.widget.clone()).downcast::<Widget>().unwrap();
	    let this = common::cast_gtk_widget_to_member_mut::<List>(&mut this).unwrap();
        if let Some(ref mut callback) = self.h_left_clicked {
            (callback.as_mut())(this, i, item_view)
        }
    }
    fn on_item_click(&mut self, cb: Option<callbacks::OnItemClick>) {
        self.h_left_clicked = cb;
    }
}
impl AdaptedInner for GtkList {
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
        self.base.invalidate();
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
    fn on_size_set(&mut self, _: &mut MemberBase, _: (u16, u16)) -> bool {
        self.base.invalidate();
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
impl Spawnable for GtkList {
    fn spawn() -> Box<dyn controls::Control> {
        Self::with_adapter(Box::new(types::imp::StringVecAdapter::<crate::imp::Text>::new())).into_control()
    }
}

fn on_size_allocate<O: controls::List>(this: &::gtk::Widget, _allo: &::gtk::Rectangle) {
    let mut ll = this.clone().upcast::<Widget>();
    let ll = common::cast_gtk_widget_to_member_mut::<List>(&mut ll).unwrap();

    let measured_size = ll.inner().base.measured;
    ll.call_on_size::<O>(measured_size.0 as u16, measured_size.1 as u16);
    
    let mut y = 0;
    let list = ll.inner_mut().inner_mut().inner_mut().inner_mut().inner_mut();
    for i in 0..list.items.len() {
        let item = &mut list.items[i];
        let (_, ch, _) = item.measure(cmp::max(0, measured_size.0 as i32) as u16, cmp::max(0, measured_size.1 as i32) as u16);
        item.draw(Some((0, y)));
        y += ch as i32;
    }
}
fn on_activated<O: controls::List>(this: &ListBox, row: &ListBoxRow) {
    let i = row.get_index();
    if i < 0 {
        return;
    }
    let mut ll = this.clone().upcast::<Widget>();
    let ll = common::cast_gtk_widget_to_member_mut::<List>(&mut ll).unwrap();
    let mut ll2 = this.clone().upcast::<Widget>();
    let ll2 = common::cast_gtk_widget_to_member_mut::<List>(&mut ll2).unwrap();
    let item_view = ll.inner_mut().inner_mut().inner_mut().inner_mut().inner_mut().items.get_mut(i as usize).unwrap();
    if let Some(ref mut callback) = ll2.inner_mut().inner_mut().inner_mut().inner_mut().inner_mut().h_left_clicked {
        let mut ll2 = this.clone().upcast::<Widget>();
        let ll2 = common::cast_gtk_widget_to_member_mut::<O>(&mut ll2).unwrap();
        (callback.as_mut())(ll2, i as usize, item_view.as_mut());
    }
}
