use std::panic;

use crate::common::{self, matrix::*, *};

use gtk::{SelectionMode, TreeSelectionExt, TreeViewExt, CellLayoutExt, ContainerExt, CellRendererExt, TreeViewColumn, TreeViewColumnExt, ListStore, ListStoreExtManual, TreeModelExt, ListStoreExt, ScrolledWindow, ScrolledWindowExt, PolicyType};
use glib::{translate::ToGlibPtrMut, signal::Inhibit};
use gobject_sys::g_value_set_pointer;

pub type Table = AMember<AControl<AContainer<AAdapted<ATable<GtkTable>>>>>;

const DEFAULT_PADDING: i32 = common::DEFAULT_PADDING / 2;

#[repr(C)]
pub struct GtkTable {
    base: GtkControlBase<Table>,
    tree_view: reckless::RecklessTreeView,
    renderer: reckless::cell_renderer::RecklessCellRenderer,
    store: ListStore,
    width: usize, height: usize,
    data: Matrix<GtkWidget>,
    h_left_clicked: Option<callbacks::OnItemClick>,
}

impl GtkTable {
    fn add_row_inner(&mut self, base: &mut MemberBase, index: usize) -> Option<&mut Row<GtkWidget>> {
        let (_, control, _, _) = unsafe { Table::adapter_base_parts_mut(base) };
        self.store.insert(index as i32);
        let row = Row {
            cells: self.data.cols.iter_mut().map(|_| None).collect(),
            native: self.base.widget.clone(), // should not be used
            control: None,
            height: self.data.default_row_height,
        };
        self.data.rows.insert(index, row);
        self.resize_row(control, index, self.data.default_row_height, true);
        self.data.row_at_mut(index)
    }
    fn remove_row_inner(&mut self, _base: &mut MemberBase, index: usize) {
        if self.store.remove(self.store.iter_nth_child(None, index as i32).as_ref().expect("The Nth iterator should exist")) {
            self.data.row_at_mut(index).map(|row| {
                (0..row.cells.len()).into_iter().for_each(|y| {
                    row.cells.remove(y).map(|cell| {
                        let widget = Object::from(cell.native.clone()).downcast::<Widget>().unwrap();
                        widget.hide();
                        widget.unparent();
                    });
                });
                row.cells.iter_mut().map(|cell| cell.as_mut().map(|cell| cell.native.clone()))
            });
            self.data.rows.remove(index);
        } else {
            panic!("Could not remove row #{}", index);
        }
    }
	fn add_column_inner(&mut self, base: &mut MemberBase, index: usize) {
        let (member, control, adapter, _) = unsafe { Table::adapter_base_parts_mut(base) };
        let (pw, ph) = control.measured;
        let width = utils::coord_to_size(pw as i32 - DEFAULT_PADDING);
        let height = utils::coord_to_size(ph as i32 - DEFAULT_PADDING);

        let this: &mut Table = unsafe { utils::base_to_impl_mut(member) };
        let indices = &[index];
        let mut item = adapter.adapter.spawn_item_view(indices, this);
        let widget = {
            let col = TreeViewColumn::new();
            col.pack_start(&this.inner_mut().inner_mut().inner_mut().inner_mut().inner_mut().renderer, false);
            col.add_attribute(&this.inner_mut().inner_mut().inner_mut().inner_mut().inner_mut().renderer, "cell", index as i32);
            item.as_mut().map(|item| {
                let widget = common::cast_control_to_gtkwidget(item.as_mut());
                let widget = Object::from(widget.clone()).downcast::<Widget>().unwrap();
                col.set_widget(Some(&widget));
                item.set_layout_width(layout::Size::Exact(width));
                item.set_layout_height(self.data.default_row_height);
                item.on_added_to_container(this, 0, 0, width, height);
                widget.show();
            }).or_else(|| adapter.adapter.alt_text_at(indices).map(|value| col.set_title(value)));
            col.set_resizable(true);
            col.set_visible(true);
            col.set_sizing(gtk::TreeViewColumnSizing::Autosize);
            //col.connect_property_width_notify(column_resized);
            GtkWidget::from(col.upcast::<glib::Object>())
        };
        self.data.cols.insert(index, Column {
            control: item,
            native: widget,
            width: layout::Size::MatchParent,
        });
        {
            let widget = Object::from(self.data.cols[index].native.clone());
            let col = widget.downcast::<TreeViewColumn>().unwrap();
            self.tree_view.insert_column(&col, index as i32);
        }
        self.resize_column(control, index, self.data.cols[index].width);
        self.data.rows.iter_mut().enumerate().for_each(|(row_index, row)| {
            row.cells.insert(index, None);
            this.inner_mut().inner_mut().inner_mut().inner_mut().inner_mut().resize_row(control, row_index, row.height, true);
        });
    }
	fn add_cell_inner(&mut self, base: &mut MemberBase, x: usize, y: usize) {
        let (member, control, adapter, _) = unsafe { Table::adapter_base_parts_mut(base) };
        let (pw, ph) = control.measured;
        if self.data.rows.len() <= y {
            self.add_row_inner(member, y);
        }
        if self.data.cols.len() <= x {
            self.add_column_inner(member, x);
        }
        let this: &mut Table = unsafe { utils::base_to_impl_mut(member) };
        adapter.adapter.spawn_item_view(&[x, y], this).map(|mut item| {
        	let gtk_widget = common::cast_control_to_gtkwidget(item.as_mut());
            let widget = Object::from(gtk_widget.clone()).downcast::<Widget>().unwrap();
            widget.set_parent(&self.tree_view);
            widget.connect_draw(|this,_| {
                this.get_parent().unwrap().queue_draw();
                Inhibit(false)
            });
            let mut width = self.data.column_at(x).map(|col| {
                let widget = Object::from(col.native.clone());
                widget.downcast::<TreeViewColumn>().unwrap().get_width()
            }).expect("Column does not exist!");
            if width >= DEFAULT_PADDING {
                width -= DEFAULT_PADDING;
            }
            self.data.rows.get_mut(y).map(|row| {
        		let mut val = Value::from_type(Type::Pointer);
	            let ptr: *mut gobject_sys::GObject = widget.to_glib_none().0;
	            unsafe { g_value_set_pointer(val.to_glib_none_mut().0, ptr as *mut c_void); }
	            let store: &mut ListStore = &mut this.inner_mut().inner_mut().inner_mut().inner_mut().inner_mut().store;
	            let iter = store.iter_nth_child(None.as_ref(), y as i32);
	            store.set_value(iter.as_ref().unwrap(), x as u32, &val);
	            item.set_layout_width(layout::Size::Exact(width as u16));
	            item.set_layout_height(row.height);
	            item.on_added_to_container(this, 0, 0, pw, ph);
	            
                row.cells.insert(y, Some(Cell {
                    control: Some(item),
                    native: gtk_widget,
                }));
            });
        }).unwrap_or_else(|| {});
    }
	fn remove_column_inner(&mut self, member: &mut MemberBase, index: usize) {
        let this: &mut Table = unsafe { utils::base_to_impl_mut(member) };
        self.data.rows.iter_mut().enumerate().for_each(|(row_index, row)| {
            //this.inner_mut().inner_mut().inner_mut().inner_mut().inner_mut().remove_cell_inner(member, row_index, index);
            let cell = if index < row.cells.len() { row.cells.remove(index) } else { None };
            cell.map(|cell| {
                cell.control.map(|mut control| control.on_removed_from_container(this));
                let widget = Object::from(cell.native.clone()).downcast::<Widget>().unwrap();
                widget.hide();
                widget.unparent();
                this.inner_mut().inner_mut().inner_mut().inner_mut().inner_mut().store.iter_nth_child(None.as_ref(), row_index as i32);
            });
        });
        let column = if index < self.data.cols.len() { Some(self.data.cols.remove(index)) } else { None };
        column.map(|column| {
            column.control.map(|mut column| column.on_removed_from_container(this));
            let col = Object::from(column.native.clone()).downcast::<TreeViewColumn>().unwrap();
            self.tree_view.remove_column(&col)
        });
    }
    fn remove_cell_inner(&mut self, member: &mut MemberBase, x: usize, y: usize) {
        let this: &mut Table = unsafe { utils::base_to_impl_mut(member) };
        self.data.rows.get_mut(x).map(|row| {
            row.cells.remove(y).map(|cell| {
                cell.control.map(|mut control| control.on_removed_from_container(this));
                let widget = Object::from(cell.native.clone()).downcast::<Widget>().unwrap();
                widget.hide();
                widget.unparent();
            });
            row.cells.insert(y, None);
        });
        let val = Value::from_type(Type::Pointer);
        let store: &mut ListStore = &mut this.inner_mut().inner_mut().inner_mut().inner_mut().inner_mut().store;
        let iter = store.iter_nth_child(None.as_ref(), y as i32);
        store.set_value(iter.as_ref().unwrap(), x as u32, &val);
    }
    fn change_column_inner(&mut self, base: &mut MemberBase, index: usize) {
        self.remove_column_inner(base, index);
        self.add_column_inner(base, index);
    }
    fn change_cell_inner(&mut self, base: &mut MemberBase, x: usize, y: usize) {
        self.remove_cell_inner(base, x, y);
        self.add_cell_inner(base, x, y);
    }
/* 
    fn update_item_inner(&mut self, base: &mut MemberBase, indexes: &[usize], node: &adapter::Node) {
    	let this: &mut Table = unsafe { utils::base_to_impl_mut(base) };
        
        let mut data = &mut self.data.0;
        let mut iter = None;
        for i in 0..indexes.len() {
            let index = indexes[i];
            let end = i+1 >= indexes.len();
            
            iter = self.store.iter_nth_child(iter.as_ref(), index as i32);

            if end {
            	data[index].expanded = if let adapter::Node::Branch(expanded) = node { *expanded } else { false };
                match data[index].node() {
                	adapter::Node::Branch(expanded) => {
                		let path = this.inner_mut().inner_mut().inner_mut().inner_mut().inner_mut().store.get_path(iter.as_ref().unwrap()).unwrap();
                		if expanded {
                			this.inner_mut().inner_mut().inner_mut().inner_mut().inner_mut().boxc.expand_row(&path, false); 
                		} else {
                			this.inner_mut().inner_mut().inner_mut().inner_mut().inner_mut().boxc.collapse_row(&path); 
                		}
                	},
                	_ => {}
                }
                return;
            } else {
                data = &mut data[index].branches;
            }
        }
    }*/
    fn resize_row(&mut self, base: &ControlBase, index: usize, size: layout::Size, force: bool) {
        let (w, h) = base.measured;
            let height = match size {
                layout::Size::Exact(height) => height,
                layout::Size::WrapContent => self.data.rows.iter()
                        .flat_map(|row| row.cells.iter())
                        .filter(|cell| cell.is_some())
                        .map(|cell| cell.as_ref().unwrap().control.as_ref())
                        .filter(|control| control.is_some())
                        .map(|control| control.unwrap().size().1)
                        .fold(0, |s, i| if s > i {s} else {i}),
                layout::Size::MatchParent => base.measured.1 / self.data.cols.len() as u16,
            };
            self.renderer.set_property_height(height as i32);
            self.data.cols.iter_mut().for_each(|col| {
                col.control.as_mut().map(|control| {
                    control.set_layout_height(layout::Size::Exact(height));
                    control.measure(w, h);
                    control.draw(None);
                });
            });
            self.data.rows.iter_mut().for_each(|row| {
                row.height = size;
                row.control.as_mut().map(|control| {
                    control.set_layout_height(layout::Size::Exact(height));
                    control.measure(w, h);
                    control.draw(None);
                });
                row.cells.iter_mut().for_each(|cell| {
                    cell.as_mut().map(|cell| {
                        cell.control.as_mut().map(|control| {
                            control.set_layout_height(layout::Size::Exact(height));
                            control.measure(w, h);
                            control.draw(None);
                        });
                    });
                });
            });
        /*if force || self.data.default_row_height != size {
            
            if !force {
                self.data.row_at_mut(index).map(|row| row.height = size);
            }
        } else {
            let row_height = self.data.default_row_height;
            self.data.row_at_mut(index).map(|mut row| row.height = row_height);
        }*/
    }
    fn resize_column(&mut self, base: &ControlBase, index: usize, size: layout::Size) {
        let (w, h) = base.measured;
        let mut width = match size {
            layout::Size::Exact(width) => width,
            layout::Size::WrapContent => self.data.rows.iter()
                    .flat_map(|row| row.cells.iter())
                    .filter(|cell| cell.is_some())
                    .map(|cell| cell.as_ref().unwrap().control.as_ref())
                    .filter(|control| control.is_some())
                    .map(|control| control.unwrap().size().0)
                    .fold(0, |s, i| if s > i {s} else {i}),
            layout::Size::MatchParent => w / self.data.cols.len() as u16,
        };
        self.tree_view.get_column(index as i32).map(|col| {
            col.set_sizing(gtk::TreeViewColumnSizing::Autosize);
            col.set_fixed_width(width as i32);
        });
        if width as i32 >= DEFAULT_PADDING {
            width = utils::coord_to_size(width as i32 - DEFAULT_PADDING);
        }
        self.data.column_at_mut(index).map(|col| {
            col.width = size;
            col.control.as_mut().map(|control| {
                control.set_layout_width(layout::Size::Exact(width));
                control.measure(w, h);
                control.draw(None);
            });
        });
        self.data.rows.iter_mut().for_each(|row| {
            row.cells.iter_mut().for_each(|cell| {
                cell.as_mut().map(|cell| {
                    cell.control.as_mut().map(|control| {
                        control.set_layout_width(layout::Size::Exact(width));
                        control.measure(h, h);
                        control.draw(None);
                    });
                });
            });
        });
    }
}
impl<O: controls::Table> NewTableInner<O> for GtkTable {
    fn with_uninit_params(ptr: &mut mem::MaybeUninit<O>, width: usize, height: usize) -> Self {
        let ptr = ptr as *mut _ as *mut c_void;
        let scr = reckless::RecklessScrolledWindow::new();
        let tv = reckless::RecklessTreeView::new();
        tv.set_halign(Align::Fill);
        tv.set_valign(Align::Fill);
        tv.get_selection().set_mode(SelectionMode::None);
        tv.set_headers_visible(true);
        scr.set_policy(PolicyType::Automatic, PolicyType::Automatic);
        scr.set_min_content_height(1);
        scr.add(&tv);
        let scr = scr.upcast::<Widget>();
        scr.connect_size_allocate(on_size_allocate::<O>);
        let mut this = GtkTable {
            base: common::GtkControlBase::with_gtk_widget(scr),
            tree_view: tv,
            renderer: reckless::cell_renderer::RecklessCellRenderer::new(),
            store: ListStore::new((0..width).into_iter().map(|_| Type::Pointer).collect::<Vec<_>>().as_slice()),
            data: Default::default(),
            h_left_clicked: None,
            width, height
        };
        this.tree_view.set_model(&this.store);
        this.renderer.set_visible(true);
        this.base.widget().show_all();
        common::set_pointer(&mut this.tree_view.clone().upcast(), ptr);
        this.base.set_pointer(ptr);  
        this
    }
}
impl TableInner for GtkTable {
    fn with_adapter_initial_size(adapter: Box<dyn types::Adapter>, width: usize, height: usize) -> Box<dyn controls::Table> {
        let mut b: Box<mem::MaybeUninit<Table>> = Box::new_uninit();
        let ab = AMember::with_inner(
            AControl::with_inner(
                AContainer::with_inner(
                    AAdapted::with_inner(
                        ATable::with_inner(
                            <Self as NewTableInner<Table>>::with_uninit_params(b.as_mut(), width, height)
                        ),
                        adapter,
                        &mut b,
                    ),
                )
            ),
        );
        let mut bb = unsafe {
	        b.as_mut_ptr().write(ab);
	        b.assume_init()
        };
        let (member, _, adapter, table) = unsafe { Table::adapter_base_parts_mut(&mut bb.base) };
        adapter.adapter.for_each(&mut (|indexes, node| {
            match node {
                adapter::Node::Leaf => table.inner_mut().add_cell_inner(member, indexes[0], indexes[1]),
                adapter::Node::Branch(_) => table.inner_mut().add_column_inner(member, indexes[0])
            }
        }));
        bb
    }
    fn set_column_width(&mut self, _: &mut MemberBase, control: &mut ControlBase, _: &mut AdaptedBase, index: usize, size: layout::Size) {
        self.resize_column(control, index, size)
    }
    fn set_row_height(&mut self, _: &mut MemberBase, control: &mut ControlBase, _: &mut AdaptedBase, index: usize, size: layout::Size) {
        self.resize_row(control, index, size, false)
    }
/*    fn resize(&mut self, member: &mut MemberBase, control: &mut ControlBase, adapted: &mut AdaptedBase, width: usize, height: usize) -> (usize, usize) {
        let old_size = self.size(member, control, adapted);
        let (max_width, max_height) = (cmp::max(width, old_size.0), cmp::max(height, old_size.1));
        let (min_width, min_height) = (cmp::min(width, old_size.0), cmp::min(height, old_size.1));
        (min_height..max_height).rev().for_each(|x| 
            if self.data.rows.len() > x {
                if old_size.0 > x {
                    self.remove_row_inner(member, x);
                }
            } else {
                if old_size.0 < x {
                     self.add_row_inner(member, x);
                }
            }
        );
        (min_width..max_width).rev().for_each(|y| 
            if self.data.cols.len() > y {
                if old_size.0 > y {
                    self.remove_column_inner(member, y);
                }
            } else {
                if old_size.0 < y {
                     self.add_column_inner(member, y, false);
                }
            }
        );
        old_size
    }*/
}
impl ItemClickableInner for GtkTable {
    fn item_click(&mut self, i: &[usize], item_view: &mut dyn controls::Control, _skip_callbacks: bool) {
        let mut this = Object::from(self.base.widget.clone()).downcast::<Widget>().unwrap();
        let this = common::cast_gtk_widget_to_member_mut::<Table>(&mut this).unwrap();
        if let Some(ref mut callback) = self.h_left_clicked {
            (callback.as_mut())(this, i, item_view)
        }
    }
    fn on_item_click(&mut self, cb: Option<callbacks::OnItemClick>) {
        self.h_left_clicked = cb;
    }
}
impl AdaptedInner for GtkTable {
    fn on_item_change(&mut self, base: &mut MemberBase, value: adapter::Change) {
        match value {
            adapter::Change::Added(at, node) => {
                if adapter::Node::Leaf == node || at.len() > 1 {
                    self.add_cell_inner(base, at[0], at[1]);
                } else {
                    self.add_column_inner(base, at[0]);
                }
            },
            adapter::Change::Removed(at) => {
                if at.len() > 1 {
                    self.remove_cell_inner(base, at[0], at[1]);
                } else {
                    self.remove_column_inner(base, at[0]);
                }
            },
            adapter::Change::Edited(at, node) => {
                if adapter::Node::Leaf == node || at.len() > 1 {
                    self.change_cell_inner(base, at[0], at[1]);
                } else {
                    self.change_column_inner(base, at[0]);
                }
            },
        }
        self.base.widget().get_toplevel().unwrap().queue_draw(); // TODO WHY????
    }
}
impl ContainerInner for GtkTable {
    fn find_control_mut<'a>(&'a mut self, arg: types::FindBy<'a>) -> Option<&'a mut dyn controls::Control> {
        for column in self.data.cols.as_mut_slice() {
            let maybe = column.control.as_mut().and_then(|control| utils::find_by_mut(control.as_mut(), arg));
            if maybe.is_some() {
                return maybe;
            }
        }
        for row in self.data.rows.as_mut_slice() {
            for cell in row.cells.as_mut_slice() {
                if let Some(cell) = cell {
                    let maybe = cell.control.as_mut().and_then(|control| utils::find_by_mut(control.as_mut(), arg));
                    if maybe.is_some() {
                        return maybe;
                    }
                }
            }
        }
        None
    }
    fn find_control<'a>(&'a self, arg: types::FindBy<'a>) -> Option<&'a dyn controls::Control> {
        for column in self.data.cols.as_slice() {
            let maybe = column.control.as_ref().and_then(|control| utils::find_by(control.as_ref(), arg));
            if maybe.is_some() {
                return maybe;
            }
        }
        for row in self.data.rows.as_slice() {
            for cell in row.cells.as_slice() {
                if let Some(cell) = cell {
                    let maybe = cell.control.as_ref().and_then(|control| utils::find_by(control.as_ref(), arg));
                    if maybe.is_some() {
                        return maybe;
                    }
                }
            }
        }
        None
    }
}

impl HasLayoutInner for GtkTable {
    fn on_layout_changed(&mut self, _base: &mut MemberBase) {
        self.base.invalidate();
    }
}

impl ControlInner for GtkTable {
    fn on_added_to_container(&mut self, member: &mut MemberBase, control: &mut ControlBase, _parent: &dyn controls::Container, x: i32, y: i32, pw: u16, ph: u16) {
        let parent = self.tree_view.clone();
        let this: &mut Table = unsafe { utils::base_to_impl_mut(member) };
        self.data.cols.iter_mut().enumerate().for_each(|(index, col)| {
            //col.control.as_mut().map(|control| set_parent(control.as_mut(), Some(&parent)));
            this.inner_mut().inner_mut().inner_mut().inner_mut().inner_mut().resize_column(control, index, col.width);
        });
        self.data.rows.iter_mut().enumerate().for_each(|(index, row)| {
            this.inner_mut().inner_mut().inner_mut().inner_mut().inner_mut().resize_row(control, index, row.height, false);
            row.control.as_mut().map(|control| set_parent(control.as_mut(), Some(&parent)));
            row.cells.iter_mut()
                .filter(|cell| cell.is_some())
                .for_each(|cell| {
                    cell.as_mut().unwrap().control.as_mut()
                        .map(|control| set_parent(control.as_mut(), Some(&parent)));
                });
        });
        self.measure(member, control, pw, ph);
        control.coords = Some((x, y));
        self.draw(member, control);
    }
    fn on_removed_from_container(&mut self, _: &mut MemberBase, _: &mut ControlBase, _: &dyn controls::Container) {
        self.data.rows.iter_mut().for_each(|row| row.cells.iter_mut()
                .filter(|cell| cell.is_some())
                .for_each(|cell| {
                    cell.as_mut().unwrap().control.as_mut()
                        .map(|control| set_parent(control.as_mut(), None));
                }));
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
        use plygui_api::markup::MEMBER_TYPE_TEXT;
        fill_from_markup_base!(self, base, markup, registry, Table, [MEMBER_TYPE_TEXT]);
        fill_from_markup_label!(self, &mut base.member, markup);
    }
}

impl HasNativeIdInner for GtkTable {
    type Id = common::GtkWidget;

    fn native_id(&self) -> Self::Id {
        self.base.widget.clone().into()
    }
}

impl HasSizeInner for GtkTable {
    fn on_size_set(&mut self, _: &mut MemberBase, _: (u16, u16)) -> bool {
        self.base.invalidate();
        true
    }
}

impl HasVisibilityInner for GtkTable {
    fn on_visibility_set(&mut self, _: &mut MemberBase, _: types::Visibility) -> bool {
        self.base.invalidate()
    }
}

impl MemberInner for GtkTable {}

impl Drawable for GtkTable {
    fn draw(&mut self, _: &mut MemberBase, control: &mut ControlBase) {
        fn draw_inner(cell: &mut Cell<GtkWidget>) {
            cell.control.as_mut().map(|control| {
                control.draw(Some((0, 0)));
            });
        }
        self.data.cols.iter_mut().for_each(|col| {
            col.control.as_mut().map(|control| {
                control.draw(Some((0, 0)));
            });
        });
        self.data.rows.iter_mut().for_each(|row| row.cells.iter_mut().filter(|cell| cell.is_some()).for_each(|cell| draw_inner(cell.as_mut().unwrap())));              
        self.base.draw(control);
        self.tree_view.set_size_request(control.measured.0 as i32, control.measured.1 as i32);
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
                let ww = if w as i32 >= DEFAULT_PADDING { utils::coord_to_size(w as i32 - DEFAULT_PADDING) } else {0};
                self.data.cols.iter_mut().for_each(|col| {
                    col.control.as_mut().map(|control| {
                        control.measure(ww, h);
                    });
                });
                fn measure_inner(cell: &mut Cell<GtkWidget>, w: u16, h: u16) {
                    cell.control.as_mut().map(|control| {
                        control.measure(w, h);
                    });
                }
                self.data.rows.iter_mut().for_each(|row| row.cells.iter_mut().filter(|cell| cell.is_some()).for_each(|cell| measure_inner(cell.as_mut().unwrap(), ww, h)));              
                (w, h)
            }
        };
        (control.measured.0, control.measured.1, control.measured != old_size)
    }
    fn invalidate(&mut self, _: &mut MemberBase, _: &mut ControlBase) {
        self.base.invalidate();
    }
}
impl Spawnable for GtkTable {
    fn spawn() -> Box<dyn controls::Control> {
        Self::with_adapter(Box::new(types::imp::StringVecAdapter::<crate::imp::Text>::new())).into_control()
    }
}

fn on_size_allocate<O: controls::Table>(this: &::gtk::Widget, _allo: &::gtk::Rectangle) {
    let mut ll = this.clone().upcast::<Widget>();
    let ll = common::cast_gtk_widget_to_member_mut::<Table>(&mut ll).unwrap();

    let measured_size = ll.inner().base.measured;
    ll.call_on_size::<O>(measured_size.0 as u16, measured_size.1 as u16);
}
fn set_parent(control: &mut dyn controls::Control, parent: Option<&reckless::RecklessTreeView>) {
    let widget = common::cast_control_to_gtkwidget(control);
    let widget = Object::from(widget.clone()).downcast::<Widget>().unwrap();
    if widget.get_parent().is_some() {
        widget.unparent();
    }
    if let Some(parent) = parent {
        widget.set_parent(parent);
    }
}
fn column_resized(tvc: &TreeViewColumn) {

}