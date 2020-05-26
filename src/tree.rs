use crate::common::{self, *};

use gtk::{TreeViewExt, ContainerExt, TreePath, TreeViewColumn, TreeStore, TreeStoreExtManual, TreeModelExt, TreeStoreExt, ScrolledWindow, ScrolledWindowExt, PolicyType};
use glib::translate::ToGlibPtrMut;
use gobject_sys::g_value_set_pointer;

pub type Tree = AMember<AControl<AContainer<AAdapted<ATree<GtkTree>>>>>;

struct TreeNode {
	node: adapter::Node,
	root: Box<dyn controls::Control>,
	branches: Vec<TreeNode>,
}

#[repr(C)]
pub struct GtkTree {
    base: GtkControlBase<Tree>,
    boxc: reckless::RecklessTreeView,
    store: TreeStore,
    items: Vec<TreeNode>,
    h_left_clicked: Option<callbacks::OnItemClick>,
}

impl GtkTree {
    fn add_item_inner(&mut self, base: &mut MemberBase, indexes: &[usize], node: adapter::Node, y: &mut i32) {
        let (member, control, adapter, _) = unsafe { Tree::adapter_base_parts_mut(base) };
        let (pw, ph) = control.measured;
        let this: &mut Tree = unsafe { utils::base_to_impl_mut(member) };
        
        let mut item = Some(adapter.adapter.spawn_item_view(indexes, node, this));
        let widget = common::cast_control_to_gtkwidget(item.as_mut().map(|item| item.as_mut()).unwrap());
        
        let mut items = &mut self.items;
        let mut iter = None;
		for i in 0..indexes.len() {
			let index = indexes[i];
			if index >= (items.len()-1) {
				items.insert(index, TreeNode {
					node: node,
    				root: if i >= (items.len() - 1) {
        					item.as_mut().map(|item| item.as_mut()).unwrap().on_added_to_container(this, 0, *y, utils::coord_to_size(pw as i32) as u16, utils::coord_to_size(ph as i32) as u16);
        					item.take().unwrap()
        				} else { 
        					adapter.adapter.spawn_item_view(
		    					&indexes[..i+1], 
		    					adapter::Node::Branch(false), 
		    					this
		    				) 
        				},
    				branches: vec![]
				});
				iter = Some(this.inner_mut().inner_mut().inner_mut().inner_mut().inner_mut().store.insert(iter.as_ref(), index as i32));
				let mut val = Value::from_type(Type::Pointer);
		        let ptr: *mut gobject_sys::GObject = Object::from(widget.clone()).to_glib_none().0;
		        unsafe { g_value_set_pointer(val.to_glib_none_mut().0, ptr as *mut c_void); }
				this.inner_mut().inner_mut().inner_mut().inner_mut().inner_mut().store.set_value(iter.as_ref().unwrap(), index as u32, &val)
			} else {
				iter = self.store.iter_nth_child(iter.as_ref(), index as i32);
				items = &mut items[index].branches;
			}
		}
    }
    fn remove_item_inner(&mut self, base: &mut MemberBase, indexes: &[usize]) {
        let this: &mut Tree = unsafe { utils::base_to_impl_mut(base) };
        let mut items = &mut self.items;
        let mut iter = None;
		for i in 0..indexes.len() {
			let index = indexes[i];
			if index >= (items.len()-1) {
				items.remove(index).root.on_removed_from_container(this); 
				this.inner_mut().inner_mut().inner_mut().inner_mut().inner_mut().store.remove(iter.as_ref().unwrap()/*, index as i32*/);
			} else {
				iter = self.store.iter_nth_child(iter.as_ref(), index as i32);
				items = &mut items[index].branches;
			}
		}
    }
}
impl<O: controls::Tree> NewTreeInner<O> for GtkTree {
    fn with_uninit(ptr: &mut mem::MaybeUninit<O>) -> Self {
        let ptr = ptr as *mut _ as *mut c_void;
        let li = reckless::RecklessScrolledWindow::new();
        let li = li.upcast::<Widget>();
        li.connect_size_allocate(on_size_allocate::<O>);
        let mut li = GtkTree {
            base: common::GtkControlBase::with_gtk_widget(li),
            boxc: reckless::RecklessTreeView::new(),
            store: TreeStore::new(&[Type::Pointer]),
            items: Vec::new(),
            h_left_clicked: None,
        };
        li.boxc.set_activate_on_single_click(true);
        li.boxc.set_halign(Align::Fill);
        li.boxc.set_valign(Align::Fill);
        li.boxc.connect_row_activated(on_activated::<O>);
        li.boxc.set_model(&li.store);
        li.boxc.show();
        let scr = Object::from(li.base.widget.clone()).downcast::<ScrolledWindow>().unwrap();
        scr.set_policy(PolicyType::Never, PolicyType::Always);
        scr.add(&li.boxc);
        scr.set_min_content_height(1);
        common::set_pointer(&mut li.boxc.clone().upcast(), ptr);
        li.base.set_pointer(ptr);  
        li
    }
}
impl TreeInner for GtkTree {
	fn with_adapter(adapter: Box<dyn types::Adapter>) -> Box<dyn controls::Tree> {
		let len = adapter.len();
        let mut b: Box<mem::MaybeUninit<Tree>> = Box::new_uninit();
        let mut ab = AMember::with_inner(
            AControl::with_inner(
                AContainer::with_inner(
                    AAdapted::with_inner(
                        ATree::with_inner(
                            <Self as NewTreeInner<Tree>>::with_uninit(b.as_mut())
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
        let (member, _, adapter, tree) = unsafe { Tree::adapter_base_parts_mut(&mut bb.base) };

		let mut y = 0;
		let mut indexes = Vec::new();
        for i in 0..adapter.adapter.len() {
        	indexes.push(i);
            tree.inner_mut().add_item_inner(member, indexes.as_slice(), adapter::Node::Branch(true), &mut y); // TODO fix adapterNode
            indexes.pop();
        }
        bb
	}
}
impl ItemClickableInner for GtkTree {
    fn item_click(&mut self, i: usize, item_view: &mut dyn controls::Control, _skip_callbacks: bool) {
        let mut this = Object::from(self.base.widget.clone()).downcast::<Widget>().unwrap();
	    let this = common::cast_gtk_widget_to_member_mut::<Tree>(&mut this).unwrap();
        if let Some(ref mut callback) = self.h_left_clicked {
            (callback.as_mut())(this, i, item_view)
        }
    }
    fn on_item_click(&mut self, cb: Option<callbacks::OnItemClick>) {
        self.h_left_clicked = cb;
    }
}
impl AdaptedInner for GtkTree {
    fn on_item_change(&mut self, base: &mut MemberBase, value: adapter::Change) {
        let mut y = 0;
        {
        	fn yadder(level: &[TreeNode], y: &mut i32) {
	        	for item in level {
	                let (_, yy) = item.root.size();
	                *y += yy as i32;
	                yadder(item.branches.as_slice(), y);
	            }
        	};            
        }
        match value {
            adapter::Change::Added(at, node) => {
                self.add_item_inner(base, at, node, &mut y);
            },
            adapter::Change::Removed(at) => {
                self.remove_item_inner(base, at);
            },
            adapter::Change::Edited(_,_) => {
            },
        }
        //self.base.widget().get_toplevel().unwrap().queue_resize(); // TODO WHY????
    }
}
impl ContainerInner for GtkTree {
    fn find_control_mut<'a>(&'a mut self, arg: &'a types::FindBy) -> Option<&'a mut dyn controls::Control> {
    	fn find_control_inner_mut<'a>(vec: &'a mut [TreeNode], arg: &'a types::FindBy) -> Option<&'a mut dyn controls::Control> {
	    	for child in vec {
	            match arg {
	                types::FindBy::Id(ref id) => {
	                    if child.root.as_member_mut().id() == *id {
	                        return Some(child.root.as_mut());
	                    }
	                }
	                types::FindBy::Tag(ref tag) => {
	                    if let Some(mytag) = child.root.as_member_mut().tag() {
	                        if tag.as_str() == mytag {
	                            return Some(child.root.as_mut());
	                        }
	                    }
	                }
	            }
	            if let Some(c) = child.root.is_container_mut() {
	                let ret = c.find_control_mut(arg);
	                if ret.is_some() {
	                    return ret;
	                }
	            }
	            let ret = find_control_inner_mut(child.branches.as_mut_slice(), arg);
	            if ret.is_some() {
	            	return ret;
	            }
	        }
	        None
    	}
    	
        find_control_inner_mut(self.items.as_mut_slice(), arg)
    }
    fn find_control<'a>(&'a self, arg: &'a types::FindBy) -> Option<&'a dyn controls::Control> {
        fn find_control_inner<'a>(vec: &'a [TreeNode], arg: &'a types::FindBy) -> Option<&'a dyn controls::Control> {
        	for child in vec {
	            match arg {
	                types::FindBy::Id(ref id) => {
	                    if child.root.as_member().id() == *id {
	                        return Some(child.root.as_ref());
	                    }
	                }
	                types::FindBy::Tag(ref tag) => {
	                    if let Some(mytag) = child.root.as_member().tag() {
	                        if tag.as_str() == mytag {
	                            return Some(child.root.as_ref());
	                        }
	                    }
	                }
	            }
	            if let Some(c) = child.root.is_container() {
	                let ret = c.find_control(arg);
	                if ret.is_some() {
	                    return ret;
	                }
	            }
	            let ret = find_control_inner(child.branches.as_slice(), arg);
	            if ret.is_some() {
	            	return ret;
	            }
	        }
	        None
        }
        
        find_control_inner(self.items.as_slice(), arg)
    }
}

impl HasLayoutInner for GtkTree {
    fn on_layout_changed(&mut self, _base: &mut MemberBase) {
        self.base.invalidate();
    }
}

impl ControlInner for GtkTree {
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
        fill_from_markup_base!(self, base, markup, registry, Tree, [MEMBER_TYPE_TEXT]);
        fill_from_markup_label!(self, &mut base.member, markup);
    }
}

impl HasNativeIdInner for GtkTree {
    type Id = common::GtkWidget;

    fn native_id(&self) -> Self::Id {
        self.base.widget.clone().into()
    }
}

impl HasSizeInner for GtkTree {
    fn on_size_set(&mut self, _: &mut MemberBase, _: (u16, u16)) -> bool {
        self.base.invalidate();
        true
    }
}

impl HasVisibilityInner for GtkTree {
    fn on_visibility_set(&mut self, _: &mut MemberBase, _: types::Visibility) -> bool {
        self.base.invalidate()
    }
}

impl MemberInner for GtkTree {}

impl Drawable for GtkTree {
    fn draw(&mut self, _: &mut MemberBase, control: &mut ControlBase) {
        let mut y = 0;
        
        fn draw_inner(vec: &mut [TreeNode], y: &mut i32) {
	        for item in vec {
	            let (_, ch) = item.root.size();
	            item.root.draw(Some((0, *y)));
	            *y += ch as i32;
	            draw_inner(item.branches.as_mut_slice(), y);
	        }
        }
        draw_inner(self.items.as_mut_slice(), &mut y);
        
        self.base.draw(control);
        self.boxc.set_size_request(control.measured.0 as i32, y as i32 * 13 / 10);
    }
    fn measure(&mut self, _: &mut MemberBase, control: &mut ControlBase, parent_width: u16, parent_height: u16) -> (u16, u16, bool) {
        let old_size = control.measured;
        control.measured = match control.visibility {
            types::Visibility::Gone => (0, 0),
            _ => {
                let mut w = 0;
                let mut h = 0;
                
                fn measure_inner(vec: &mut [TreeNode], parent_width: u16, parent_height: u16, w: &mut u16, h: &mut u16) {
	                for item in vec {
	                    let (cw, ch, _) = item.root.measure(cmp::max(0, parent_width as i32) as u16, cmp::max(0, parent_height as i32 - *h as i32) as u16);
	                    *w = cmp::max(*w, cw);
	                    *h += ch;
	                    measure_inner(item.branches.as_mut_slice(), parent_width, parent_height, w, h);
	                }
                }
                measure_inner(self.items.as_mut_slice(), parent_width, parent_height, &mut w, &mut h);
                
                let w = match control.layout.width {
                    layout::Size::MatchParent => parent_width,
                    layout::Size::Exact(w) => w,
                    layout::Size::WrapContent => cmp::max(0, w as i32) as u16,
                };
                let h = match control.layout.height {
                    layout::Size::MatchParent => parent_height,
                    layout::Size::Exact(h) => h,
                    layout::Size::WrapContent => cmp::max(0, h as i32) as u16,
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
impl Spawnable for GtkTree {
    fn spawn() -> Box<dyn controls::Control> {
        Self::with_adapter(Box::new(types::imp::StringVecAdapter::<crate::imp::Text>::new())).into_control()
    }
}

fn on_size_allocate<O: controls::Tree>(this: &::gtk::Widget, _allo: &::gtk::Rectangle) {
    let mut ll = this.clone().upcast::<Widget>();
    let ll = common::cast_gtk_widget_to_member_mut::<Tree>(&mut ll).unwrap();

    let measured_size = ll.inner().base.measured;
    ll.call_on_size::<O>(measured_size.0 as u16, measured_size.1 as u16);
}
fn on_activated<O: controls::Tree>(this: &reckless::RecklessTreeView, path: &TreePath, column: &TreeViewColumn) {
    /*let i = path.get_indices_with_depth();
    if i < 0 {
        return;
    }
    let mut ll = this.clone().upcast::<Widget>();
    let ll = common::cast_gtk_widget_to_member_mut::<Tree>(&mut ll).unwrap();
    let mut ll2 = this.clone().upcast::<Widget>();
    let ll2 = common::cast_gtk_widget_to_member_mut::<Tree>(&mut ll2).unwrap();
    let item_view = ll.inner_mut().inner_mut().inner_mut().inner_mut().inner_mut().items.get_mut(i as usize).unwrap();
    if let Some(ref mut callback) = ll2.inner_mut().inner_mut().inner_mut().inner_mut().inner_mut().h_left_clicked {
        let mut ll2 = this.clone().upcast::<Widget>();
        let ll2 = common::cast_gtk_widget_to_member_mut::<O>(&mut ll2).unwrap();
        (callback.as_mut())(ll2, i as usize, item_view.as_mut());
    }*/
}
