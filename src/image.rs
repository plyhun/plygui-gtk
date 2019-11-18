use crate::common::{self, *};

use gtk::{Cast, Image as GtkImageSys, ImageExt, Widget, WidgetExt};

pub type Image = Member<Control<GtkImage>>;

#[repr(C)]
pub struct GtkImage {
    base: GtkControlBase<Image>,

    scale: types::ImageScalePolicy,
    orig: Pixbuf,
}

impl ImageInner for GtkImage {
    fn with_content(content: image::DynamicImage) -> Box<dyn controls::Image> {
        let pixbuf = common::image_to_pixbuf(&content);

        let mut i = Box::new(Member::with_inner(
            Control::with_inner(
                GtkImage {
                    base: GtkControlBase::with_gtk_widget(GtkImageSys::new_from_pixbuf(Some(&pixbuf)).upcast::<Widget>()),
                    scale: types::ImageScalePolicy::FitCenter,
                    orig: pixbuf,
                },
                (),
            ),
            MemberFunctions::new(_as_any, _as_any_mut, _as_member, _as_member_mut),
        ));
        i.as_inner_mut().as_inner_mut().base.widget().connect_size_allocate(on_size_allocate);
        i.as_inner_mut().as_inner_mut().base.widget().connect_show(on_show);
        {
            let ptr = i.as_ref() as *const _ as *mut ::std::os::raw::c_void;
            i.as_inner_mut().as_inner_mut().base.set_pointer(ptr);
        }
        i
    }
    fn set_scale(&mut self, _: &mut MemberBase, policy: types::ImageScalePolicy) {
        if self.scale != policy {
            self.scale = policy;
            self.base.invalidate();
        }
    }
    fn scale(&self) -> types::ImageScalePolicy {
        self.scale
    }
}

impl GtkImage {
    fn apply_sized_image(&mut self, control: &ControlBase) {
        let bm_width = self.orig.get_width();
        let bm_height = self.orig.get_height();

        let (aw, ah) = control.measured;
        let (lm, tm, rm, bm) = self.base.margins().into();
        let hoffs = lm;
        let voffs = tm;
        let hdiff = hoffs + rm;
        let vdiff = voffs + bm;
        let inner_h = aw as i32 - hdiff;
        let inner_v = ah as i32 - vdiff;

        let (wrate, hrate) = (inner_h as f32 / bm_width as f32, inner_v as f32 / bm_height as f32);
        let less_rate = fmin(wrate, hrate);
        let scaled = match self.scale {
            types::ImageScalePolicy::FitCenter => {
                let bm_h = (bm_width as f32 * less_rate) as i32;
                let bm_v = (bm_height as f32 * less_rate) as i32;
                if bm_h < 1 || bm_v < 1 {
                    return;
                }
                let alpha = self.orig.get_has_alpha();
                let bits = self.orig.get_bits_per_sample();

                let scaled = Pixbuf::new(Colorspace::Rgb, alpha, bits, bm_h, bm_v);
                self.orig.scale(&scaled, 0, 0, bm_h, bm_v, 0f64, 0f64, less_rate as f64, less_rate as f64, InterpType::Hyper);
                scaled
            }
            types::ImageScalePolicy::CropCenter => {
                let half_diff_h = (bm_width - aw as i32) / 2;
                let half_diff_v = (bm_height - ah as i32) / 2;
                self.orig.new_subpixbuf(cmp::max(0, half_diff_h), cmp::max(0, half_diff_v), cmp::min(bm_width, inner_h), cmp::min(bm_height, inner_v)).unwrap()
            }
        };
        Object::from(self.base.widget.clone()).downcast::<GtkImageSys>().unwrap().set_from_pixbuf(&scaled);
    }
}

impl HasLayoutInner for GtkImage {
    fn on_layout_changed(&mut self, _: &mut MemberBase) {
        //self.apply_padding(unsafe { &mut utils::member_control_base_mut_unchecked(base).control });
        self.base.invalidate();
    }
}

impl ControlInner for GtkImage {
    fn on_added_to_container(&mut self, member: &mut MemberBase, control: &mut ControlBase, _parent: &dyn controls::Container, x: i32, y: i32, pw: u16, ph: u16) {
        self.measure(member, control, pw, ph);
        //self.apply_sized_image(base);
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
        use plygui_api::markup::MEMBER_TYPE_IMAGE;
        fill_from_markup_base!(self, base, markup, registry, Image, [MEMBER_TYPE_IMAGE]);
    }
}
impl HasNativeIdInner for GtkImage {
    type Id = common::GtkWidget;

    unsafe fn native_id(&self) -> Self::Id {
        self.base.widget.clone().into()
    }
}

impl HasSizeInner for GtkImage {
    fn on_size_set(&mut self, _: &mut MemberBase, (width, height): (u16, u16)) -> bool {
        self.base.widget().set_size_request(width as i32, height as i32);
        true
    }
}

impl HasVisibilityInner for GtkImage {
    fn on_visibility_set(&mut self, _: &mut MemberBase, _: types::Visibility) -> bool {
        self.base.invalidate()
    }
}

impl MemberInner for GtkImage {}

impl Drawable for GtkImage {
    fn draw(&mut self, _: &mut MemberBase, control: &mut ControlBase) {
        self.base.draw(control);
    }
    fn measure(&mut self, _: &mut MemberBase, control: &mut ControlBase, parent_width: u16, parent_height: u16) -> (u16, u16, bool) {
        let old_size = control.measured;
        control.measured = match control.visibility {
            types::Visibility::Gone => (0, 0),
            _ => {
                let (lm, tm, rm, bm) = self.base.margins().into();

                let mut size = (-1i32, -1i32);
                let w = match control.layout.width {
                    layout::Size::MatchParent => parent_width as i32,
                    layout::Size::Exact(w) => w as i32,
                    layout::Size::WrapContent => {
                        if size.0 < 0 {
                            size = (self.orig.get_width(), self.orig.get_height());
                        }
                        size.0 + lm + rm
                    }
                };
                let h = match control.layout.height {
                    layout::Size::MatchParent => parent_height as i32,
                    layout::Size::Exact(h) => h as i32,
                    layout::Size::WrapContent => {
                        if size.1 < 0 {
                            size = (self.orig.get_width(), self.orig.get_height());
                        }
                        size.1 + tm + bm
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

/*#[allow(dead_code)]
pub(crate) fn spawn() -> Box<controls::Control> {
    Image::with_label("").into_control()
}*/

fn on_show(this: &::gtk::Widget) {
    let mut ll1 = this.clone().upcast::<Widget>();
    let mut ll2 = this.clone().upcast::<Widget>();
    let ll1 = cast_gtk_widget_to_member_mut::<Image>(&mut ll1).unwrap();
    let ll2 = cast_gtk_widget_to_member_mut::<Image>(&mut ll2).unwrap();

    ll1.as_inner_mut().as_inner_mut().apply_sized_image(ll2.as_inner().base());
}

fn on_size_allocate(this: &::gtk::Widget, _allo: &::gtk::Rectangle) {
    let mut ll1 = this.clone().upcast::<Widget>();
    let mut ll2 = this.clone().upcast::<Widget>();
    let ll1 = cast_gtk_widget_to_member_mut::<Image>(&mut ll1).unwrap();
    let ll2 = cast_gtk_widget_to_member_mut::<Image>(&mut ll2).unwrap();

    ll1.as_inner_mut().as_inner_mut().apply_sized_image(ll2.as_inner().base());

    let measured_size = ll1.as_inner().base().measured;
    ll1.call_on_size(measured_size.0 as u16, measured_size.1 as u16);
}

fn fmin(a: f32, b: f32) -> f32 {
    if a < b {
        a
    } else {
        b
    }
}

default_impls_as!(Image);
