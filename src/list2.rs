use crate::common::{self, *};

use glib::translate::ToGlibPtrMut;
use gobject_sys::g_value_set_pointer;
use gtk::{TreeView, TreeViewExt, TreeViewColumn, ListStore, Type, CellLayoutExt, ListStoreExtManual, Value};

pub type List = Member<Control<Adapter<GtkList>>>;

#[repr(C)]
pub struct GtkList {
    base: GtkControlBase<List>,
    col: TreeViewColumn,
    renderer: reckless::cell_renderer::RecklessCellRenderer,
    store: ListStore,
    items: Vec<Box<dyn controls::Control>>,
}

impl GtkList {
    fn add_item_inner(&mut self, base: &mut MemberBase, i: usize, y: &mut i32) {
        let (member, control, adapter) = List::adapter_base_parts_mut(base);
        let (pw, ph) = control.measured;
        let this: &mut List = unsafe { utils::base_to_impl_mut(member) };
        
        let mut item = adapter.adapter.spawn_item_view(i, this);
        item.on_added_to_container(this, 0, *y, utils::coord_to_size(pw as i32 /*- scroll_width - 14*/ /*TODO: WHY???*/ - DEFAULT_PADDING) as u16, utils::coord_to_size(ph as i32) as u16);
        let widget = common::cast_control_to_gtkwidget(item.as_mut());
                
        let (_, yy) = item.size();
        self.items.push(item);
        *y += yy as i32;
        
        let mut val = Value::from_type(Type::Pointer);
        let ptr: *mut gobject_sys::GObject = Object::from(widget.clone()).to_glib_none().0;
        unsafe { g_value_set_pointer(val.to_glib_none_mut().0, ptr as *mut c_void); }
        this.as_inner_mut().as_inner_mut().as_inner_mut().store.insert_with_values(Some(i as u32), &[0], &[&val]);
    }
    fn remove_item_inner(&mut self, base: &mut MemberBase, i: usize) {
        let this: &mut List = unsafe { utils::base_to_impl_mut(base) };
        self.items.remove(i).on_removed_from_container(this); 
        
    }
}

impl AdapterViewInner for GtkList {
	fn with_adapter(adapter: Box<dyn types::Adapter>) -> Box<Member<Control<Adapter<Self>>>> {
		let mut btn = Box::new(Member::with_inner(
            Control::with_inner(
                Adapter::with_inner(
	                GtkList {
	                    base: common::GtkControlBase::with_gtk_widget(reckless::RecklessTreeView::new().upcast::<Widget>()),
	                    col: TreeViewColumn::new(),
	                    renderer: reckless::cell_renderer::RecklessCellRenderer::new(),
	                    store: ListStore::new(&[Type::Pointer]),
	                    items: Vec::new(),
	                },
	                adapter
                ),
                (),
            ),
            MemberFunctions::new(_as_any, _as_any_mut, _as_member, _as_member_mut),
        ));
        {
            let ptr = btn.as_ref() as *const _ as *mut std::os::raw::c_void;
            btn.as_inner_mut().as_inner_mut().as_inner_mut().base.set_pointer(ptr);
        }
        btn.as_inner_mut().as_inner_mut().as_inner_mut().base.widget().connect_size_allocate(on_size_allocate);
        {
            let tv = btn.as_inner_mut().as_inner_mut().as_inner_mut().base.widget().downcast::<TreeView>().unwrap();
            let renderer = &btn.as_inner().as_inner().as_inner().renderer;
            let col = &btn.as_inner().as_inner().as_inner().col;
            col.pack_start(renderer, false);
            col.add_attribute(renderer, "cell", 0);
            tv.set_model(&btn.as_inner_mut().as_inner_mut().as_inner_mut().store);
            tv.append_column(&btn.as_inner_mut().as_inner_mut().as_inner_mut().col);
        }
        btn
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
    fn on_size_set(&mut self, _: &mut MemberBase, (width, height): (u16, u16)) -> bool {
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
}

default_impls_as!(List);
