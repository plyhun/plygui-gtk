#[macro_use]
extern crate plygui_api;
#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate glib;
extern crate gdk;
extern crate gtk;
extern crate libc;
extern crate pango;

extern crate glib_sys;
extern crate gobject_sys;
extern crate gtk_sys;

pub mod reckless;

#[macro_use]
pub mod common;

mod application;
mod button;
mod frame;
mod layout_linear;
mod splitted;
mod window;

pub use self::application::Application;
pub use self::button::Button;
pub use self::frame::Frame;
pub use self::layout_linear::LinearLayout;
pub use self::splitted::Splitted;
pub use self::window::Window;

#[cfg(feature = "markup")]
pub fn register_members(registry: &mut plygui_api::markup::MarkupRegistry) {
    //registry.insert(plygui_api::members::MEMBER_ID_BUTTON.into(), button::spawn);
    //registry.insert(plygui_api::members::MEMBER_ID_LAYOUT_LINEAR.into(), layout_linear::spawn);
    registry.register_member(plygui_api::markup::MEMBER_TYPE_BUTTON.into(), button::spawn);
    registry.register_member(plygui_api::markup::MEMBER_TYPE_LINEAR_LAYOUT.into(), layout_linear::spawn);
}
