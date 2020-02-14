pub use plygui_api::development::*;
pub use plygui_api::{callbacks, controls, defaults, ids, layout, types, utils};

pub use glib::translate::ToGlibPtr;
pub use glib::Object;
pub use gobject_sys::GObject;
pub use gtk::{Align, Cast, Menu as GtkMenu, MenuItem as GtkMenuItem, MenuItemExt, MenuShell as GtkMenuShell, MenuShellExt, Orientation as GtkOrientation, SeparatorMenuItem as GtkSeparatorMenuItem, Widget, WidgetExt};
pub use gtk_sys::GtkWidget as WidgetSys;
pub use gdk_pixbuf::{Colorspace, InterpType, Pixbuf, PixbufExt};
pub use cairo::{self, Format};
pub use gdk;

pub use std::borrow::Cow;
pub use std::ffi::CString;
pub use std::marker::PhantomData;
pub use std::os::raw::{c_char, c_void};
pub use std::{cmp, mem, ops, ptr, sync::mpsc};

pub use crate::reckless;
pub use crate::external::image;

lazy_static! {
    pub static ref PROPERTY: CString = CString::new("plygui").unwrap();
}

pub const DEFAULT_PADDING: i32 = 6;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GtkWidget(Object);

impl From<Object> for GtkWidget {
    fn from(a: Object) -> GtkWidget {
        GtkWidget(a)
    }
}
impl From<GtkWidget> for Object {
    fn from(a: GtkWidget) -> Object {
        a.0
    }
}
impl From<GtkWidget> for usize {
    fn from(a: GtkWidget) -> usize {
        let aa: *mut GObject = a.0.to_glib_full();
        aa as usize
    }
}
impl cmp::PartialOrd for GtkWidget {
    fn partial_cmp(&self, other: &GtkWidget) -> Option<cmp::Ordering> {
        pointer(&self.0).partial_cmp(&pointer(&other.0))
    }
}
impl cmp::Ord for GtkWidget {
    fn cmp(&self, other: &GtkWidget) -> cmp::Ordering {
        pointer(&self.0).cmp(&pointer(&other.0))
    }
}
impl ops::Deref for GtkWidget {
    type Target = Object;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ops::DerefMut for GtkWidget {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl AsRef<Object> for GtkWidget {
    fn as_ref(&self) -> &Object {
        &self.0
    }
}
impl AsMut<Object> for GtkWidget {
    fn as_mut(&mut self) -> &mut Object {
        &mut self.0
    }
}
impl NativeId for GtkWidget {
	unsafe fn from_outer(a: usize) -> GtkWidget {
		use glib::translate::FromGlibPtrFull;

        GtkWidget(Object::from_glib_full(a as *mut GObject))
	}
}

#[repr(C)]
pub struct GtkControlBase<T: controls::Control + Sized> {
    pub widget: GtkWidget,
    _marker: PhantomData<T>,
}

impl<T: controls::Control + Sized> GtkControlBase<T> {
    pub fn with_gtk_widget(widget: Widget) -> GtkControlBase<T> {
        let base = GtkControlBase {
            widget: widget.upcast::<Object>().into(),
            _marker: PhantomData,
        };
        base
    }
    pub fn set_pointer(&mut self, ptr: *mut c_void) {
        set_pointer(&mut self.widget, ptr)
    }
    pub fn pointer(&self) -> *mut c_void {
        pointer(&self.widget)
    }
    pub fn margins(&self) -> layout::BoundarySize {
        let widget = self.widget();
        layout::BoundarySize::Distinct(widget.get_margin_start(), widget.get_margin_top(), widget.get_margin_end(), widget.get_margin_bottom())
    }
    pub fn parent(&self) -> Option<&MemberBase> {
        if let Some(w) = self.widget().get_parent() {
            if pointer(&w.clone().upcast()).is_null() {
                w.get_parent().map(|w| unsafe { cast_gobject(&w.upcast()).unwrap() })
            } else {
                Some(unsafe { cast_gobject(&w.upcast()).unwrap() })
            }
        } else {
            None
        }
    }
    pub fn parent_mut(&mut self) -> Option<&mut MemberBase> {
        if let Some(w) = self.widget().get_parent() {
            if pointer(&w.clone().upcast()).is_null() {
                w.get_parent().map(|w| unsafe { cast_gobject_mut(&mut w.upcast()).unwrap() })
            } else {
                Some(unsafe { cast_gobject_mut(&mut w.upcast()).unwrap() })
            }
        } else {
            None
        }
    }
    pub fn root(&self) -> Option<&MemberBase> {
        self.widget().get_toplevel().map(|w| unsafe { cast_gobject(&w.upcast()).unwrap() })
    }
    pub fn root_mut(&mut self) -> Option<&mut MemberBase> {
        self.widget().get_toplevel().map(|w| unsafe { cast_gobject_mut(&mut w.upcast()).unwrap() })
    }
    pub fn invalidate(&mut self) -> bool {
        let widget = self.widget();
        if let Some(mut parent_widget) = widget.get_parent() {
            if pointer(&parent_widget.clone().upcast()).is_null() {
                parent_widget = parent_widget.get_parent().unwrap();
            }
            if let Some(mparent) = cast_gtk_widget_to_base_mut(&mut parent_widget) {
                let (pw, ph) = mparent.as_member().is_has_size().unwrap().size();
                let this: &mut T = unsafe { cast_gobject_mut(&mut self.widget).unwrap() };
                let (_, _, changed) = this.measure(pw, ph);
                this.draw(None);

                if let Some(cparent) = mparent.as_member_mut().is_control_mut() {
                    if changed && !cparent.is_skip_draw() {
                        cparent.invalidate();
                    }
                }
            }
            true
        } else {
            false
        }
    }
    pub fn draw(&mut self, control: &mut ControlBase) {
        if control.coords.is_some() {
            let widget = self.widget();
            widget.set_size_request(control.measured.0 as i32, control.measured.1 as i32);
            if let types::Visibility::Gone = control.visibility {
                widget.hide();
            } else {
                widget.show();
            }
            if let types::Visibility::Invisible = control.visibility {
                widget.set_sensitive(false);
                widget.set_opacity(0.0);
            } else {
                widget.set_sensitive(true);
                widget.set_opacity(1.0);
            }
        }
    }
    pub fn measure(&mut self, control: &mut ControlBase, parent_width: u16, parent_height: u16) -> (u16, u16, bool) {
        let old_size = control.measured;
        control.measured = match control.visibility {
            types::Visibility::Gone => (0, 0),
            _ => {
                let widget = self.widget();
                let native_size = gtk_allocation_to_size(&widget);
                let w = match control.layout.width {
                    layout::Size::MatchParent => parent_width as i32,
                    layout::Size::Exact(w) => w as i32,
                    layout::Size::WrapContent => native_size.0,
                };
                let h = match control.layout.height {
                    layout::Size::MatchParent => parent_height as i32,
                    layout::Size::Exact(h) => h as i32,
                    layout::Size::WrapContent => native_size.1,
                };
                (cmp::max(0, w) as u16, cmp::max(0, h) as u16)
            }
        };
        (control.measured.0, control.measured.1, control.measured != old_size)
    }
    pub fn widget(&self) -> Widget {
        Object::from(self.widget.clone()).downcast().unwrap()
    }
    pub fn as_control(&self) -> &T {
        unsafe { cast_gobject(&self.widget).unwrap() }
    }
    pub fn as_control_mut(&mut self) -> &mut T {
        unsafe { cast_gobject_mut(&mut self.widget).unwrap() }
    }
}

pub fn set_pointer(this: &mut Object, ptr: *mut c_void) {
    unsafe {
        ::gobject_sys::g_object_set_data(this.to_glib_none().0, PROPERTY.as_ptr() as *const c_char, ptr as *mut ::libc::c_void);
    }
}
pub fn pointer(this: &Object) -> *mut c_void {
    unsafe { ::gobject_sys::g_object_get_data(this.to_glib_none().0, PROPERTY.as_ptr() as *const c_char) as *mut c_void }
}
pub fn cast_member_to_gtkwidget(member: &dyn controls::Member) -> GtkWidget {
    unsafe { GtkWidget::from_outer(member.native_id()) }
}
pub fn cast_control_to_gtkwidget(control: &dyn controls::Control) -> GtkWidget {
    cast_member_to_gtkwidget(control.as_member())
}

pub unsafe fn cast_gobject_mut<'a, T>(this: &mut Object) -> Option<&'a mut T>
where
    T: Sized,
{
    let ptr = pointer(this);
    if !ptr.is_null() {
        Some(::std::mem::transmute(ptr))
    } else {
        None
    }
}
pub unsafe fn cast_gobject<'a, T>(this: &Object) -> Option<&'a T>
where
    T: Sized,
{
    let ptr = pointer(this);
    if !ptr.is_null() {
        Some(::std::mem::transmute(ptr))
    } else {
        None
    }
}
pub fn cast_gtk_widget_to_member_mut<'a, T>(object: & mut Widget) -> Option<&'a mut T>
where
    T: controls::Member + Sized,
{
    let mut object = object.clone().upcast::<Object>();
    unsafe { cast_gobject_mut(&mut object) }
}
pub fn cast_gtk_widget_to_member<'a, T>(object: & Widget) -> Option<&'a T>
where
    T: controls::Member + Sized,
{
    let object = object.clone().upcast::<Object>();
    unsafe { cast_gobject(&object) }
}
pub fn cast_gtk_widget_to_base_mut<'a>(object: &'a mut Widget) -> Option<&'a mut MemberBase> {
    let mut object = object.clone().upcast::<Object>();
    unsafe { cast_gobject_mut(&mut object) }
}
pub fn cast_gtk_widget_to_base<'a>(object: &'a Widget) -> Option<&'a MemberBase> {
    let object = object.clone().upcast::<Object>();
    unsafe { cast_gobject(&object) }
}
pub fn orientation_to_gtk(a: layout::Orientation) -> GtkOrientation {
    match a {
        layout::Orientation::Horizontal => GtkOrientation::Horizontal,
        layout::Orientation::Vertical => GtkOrientation::Vertical,
    }
}
pub fn gtk_to_orientation(a: GtkOrientation) -> layout::Orientation {
    match a {
        GtkOrientation::Horizontal => layout::Orientation::Horizontal,
        GtkOrientation::Vertical => layout::Orientation::Vertical,
        _ => panic!("Unsupported GtkOrientation"),
    }
}
pub fn gtk_allocation_to_size<'a>(object: &'a Widget) -> (i32, i32) {
    object.queue_draw();
    let alloc = object.get_allocation();
    (alloc.width, alloc.height)
}
pub fn image_to_pixbuf(src: &image::DynamicImage) -> Pixbuf {
    use image::GenericImageView;
    
    let (w, h) = src.dimensions();
    let raw = src.to_rgba().into_raw();
    let stride = Format::ARgb32.stride_for_width(w).unwrap();
    Pixbuf::new_from_vec(raw, Colorspace::Rgb, true, 8, w as i32, h as i32, stride)
}
fn append_item<T: controls::Member>(menu: GtkMenuShell, label: String, action: callbacks::Action, storage: &mut Vec<callbacks::Action>, item_spawn: fn(id: usize, selfptr: *mut T) -> GtkMenuItem, selfptr: *mut T) {
    let id = storage.len();
    let mi = item_spawn(id, selfptr);
    mi.set_label(label.as_str());
    storage.push(action);
    menu.append(&mi);
}
fn append_level<T: controls::Member>(menu: GtkMenuShell, label: String, items: Vec<types::MenuItem>, storage: &mut Vec<callbacks::Action>, item_spawn: fn(id: usize, selfptr: *mut T) -> GtkMenuItem, selfptr: *mut T) {
    let mi = GtkMenuItem::new_with_label(label.as_str());
    let submenu = GtkMenu::new();
    make_menu(submenu.clone().upcast(), items, storage, item_spawn, selfptr);
    mi.set_submenu(&submenu);
    menu.append(&mi);
}
pub fn make_menu<T: controls::Member>(menu: GtkMenuShell, mut items: Vec<types::MenuItem>, storage: &mut Vec<callbacks::Action>, item_spawn: fn(id: usize, selfptr: *mut T) -> GtkMenuItem, selfptr: *mut T) {
    let mut options = Vec::new();
    let mut help = Vec::new();

    let make_special = |menu: GtkMenuShell, mut special: Vec<types::MenuItem>, storage: &mut Vec<callbacks::Action>| {
        for item in special.drain(..) {
            match item {
                types::MenuItem::Action(label, action, _) => {
                    append_item(menu.clone(), label, action, storage, item_spawn, selfptr);
                }
                types::MenuItem::Sub(label, items, _) => {
                    append_level(menu.clone(), label, items, storage, item_spawn, selfptr);
                }
                types::MenuItem::Delimiter => {
                    menu.append(&GtkSeparatorMenuItem::new());
                }
            }
        }
    };

    for item in items.drain(..) {
        match item {
            types::MenuItem::Action(label, action, role) => match role {
                types::MenuItemRole::None => {
                    append_item(menu.clone(), label, action, storage, item_spawn, selfptr);
                }
                types::MenuItemRole::Options => {
                    options.push(types::MenuItem::Action(label, action, role));
                }
                types::MenuItemRole::Help => {
                    help.push(types::MenuItem::Action(label, action, role));
                }
            },
            types::MenuItem::Sub(label, items, role) => match role {
                types::MenuItemRole::None => {
                    append_level(menu.clone(), label, items, storage, item_spawn, selfptr);
                }
                types::MenuItemRole::Options => {
                    options.push(types::MenuItem::Sub(label, items, role));
                }
                types::MenuItemRole::Help => {
                    help.push(types::MenuItem::Sub(label, items, role));
                }
            },
            types::MenuItem::Delimiter => {
                menu.append(&GtkSeparatorMenuItem::new());
            }
        }
    }

    make_special(menu.clone(), options, storage);
    make_special(menu.clone(), help, storage);
}
