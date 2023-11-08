use crate::common::{self, *};

use gtk::{Label};
use gtk::traits::{LabelExt, LayoutExt};

use std::borrow::Cow;

pub type Text = AMember<AControl<AText<GtkText>>>;

#[repr(C)]
pub struct GtkText {
    base: GtkControlBase<Text>,
}
impl<O: controls::Text> NewTextInner<O> for GtkText {
    fn with_uninit(ptr: &mut mem::MaybeUninit<O>) -> Self {
        let ptr = ptr as *mut _ as *mut c_void;
        let tx = reckless::RecklessLabel::new();
        let tx = tx.upcast::<Widget>();
        tx.connect_size_allocate(on_size_allocate::<O>);
        let mut tx = GtkText {
            base: common::GtkControlBase::with_gtk_widget(tx),
        };
        tx.base.set_pointer(ptr);    
        tx
    }
}
impl TextInner for GtkText {
    fn with_text<S: AsRef<str>>(text: S) -> Box<dyn controls::Text> {
        let mut b: Box<mem::MaybeUninit<Text>> = Box::new_uninit();
        let mut ab = AMember::with_inner(
            AControl::with_inner(
                AText::with_inner(
                    <Self as NewTextInner<Text>>::with_uninit(b.as_mut()),
                )
            ),
        );
        controls::HasLabel::set_label(&mut ab, text.as_ref().into());
        unsafe {
	        b.as_mut_ptr().write(ab);
	        b.assume_init()
        }
    }
}

impl HasLabelInner for GtkText {
    fn label<'a>(&'a self, _: &MemberBase) -> Cow<str> {
        Cow::Owned(self.base.widget().downcast::<Label>().unwrap().text().into())
    }
    fn set_label(&mut self, _: &mut MemberBase, label: Cow<str>) {
        self.base.widget().downcast::<Label>().unwrap().set_text(&label)
    }
}

impl HasLayoutInner for GtkText {
    fn on_layout_changed(&mut self, _base: &mut MemberBase) {
        self.base.invalidate();
    }
}

impl ControlInner for GtkText {
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
        fill_from_markup_base!(self, base, markup, registry, Text, [MEMBER_TYPE_TEXT]);
        fill_from_markup_label!(self, &mut base.member, markup);
    }
}

impl HasNativeIdInner for GtkText {
    type Id = common::GtkWidget;

    fn native_id(&self) -> Self::Id {
        self.base.widget.clone().into()
    }
}

impl HasSizeInner for GtkText {
    fn on_size_set(&mut self, _: &mut MemberBase, (width, height): (u16, u16)) -> bool {
        self.base.widget().set_size_request(width as i32, height as i32);
        true
    }
}

impl HasVisibilityInner for GtkText {
    fn on_visibility_set(&mut self, _: &mut MemberBase, _: types::Visibility) -> bool {
        self.base.invalidate()
    }
}

impl MemberInner for GtkText {}

impl Drawable for GtkText {
    fn draw(&mut self, _: &mut MemberBase, control: &mut ControlBase) {
        self.base.draw(control);
    }
    fn measure(&mut self, _: &mut MemberBase, control: &mut ControlBase, parent_width: u16, parent_height: u16) -> (u16, u16, bool) {
        let old_size = control.measured;
        control.measured = match control.visibility {
            types::Visibility::Gone => (0, 0),
            _ => {
                let mut label_size = (-1i32, -1i32);

                let w = match control.layout.width {
                    layout::Size::MatchParent => parent_width as i32,
                    layout::Size::Exact(w) => w as i32,
                    layout::Size::WrapContent => {
                        let self_widget = self.base.widget();
                        if label_size.0 < 0 {
                            let label = Object::from(self.base.widget.clone()).downcast::<Label>().unwrap();
                            label_size = label.layout().unwrap().pixel_size();
                        }
                        label_size.0 + self_widget.margin_start() + self_widget.margin_end()
                    }
                };
                let h = match control.layout.height {
                    layout::Size::MatchParent => parent_height as i32,
                    layout::Size::Exact(h) => h as i32,
                    layout::Size::WrapContent => {
                        let self_widget = self.base.widget();
                        if label_size.1 < 0 {
                            let label = Object::from(self.base.widget.clone()).downcast::<Label>().unwrap();
                            label_size = label.layout().unwrap().pixel_size();
                        }
                        label_size.1 + self_widget.margin_top() + self_widget.margin_bottom()
                    }
                };
                (cmp::max(0, w) as u16, cmp::max(0, h) as u16)
            }
        };
        (control.measured.0, control.measured.1, control.measured != old_size)
    }
    fn invalidate(&mut self, _: &mut MemberBase, _: &mut ControlBase) {
        self.base.invalidate();
    }
}
impl Spawnable for GtkText {
    fn spawn() -> Box<dyn controls::Control> {
        Self::with_text("").into_control()
    }
}

fn on_size_allocate<O: controls::Text>(this: &::gtk::Widget, _allo: &::gtk::Rectangle) {
    let mut ll = this.clone().upcast::<Widget>();
    let ll = common::cast_gtk_widget_to_member_mut::<Text>(&mut ll).unwrap();

    let measured_size = ll.inner().base.measured;
    ll.call_on_size::<O>(measured_size.0 as u16, measured_size.1 as u16);
}

