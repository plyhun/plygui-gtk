#[macro_use]
extern crate plygui_api;
#[macro_use]
extern crate lazy_static;

extern crate gtk;
extern crate gdk;

#[macro_use]
pub mod common;

mod application;
mod window;
//mod button;
//mod layout_linear;

pub use self::application::Application;
pub use self::window::Window;
//pub use self::button::Button;
//pub use self::layout_linear::LinearLayout;

/*#[cfg(feature = "markup")]
pub fn register_members(registry: &mut plygui_api::markup::MarkupRegistry) {
	//registry.insert(plygui_api::members::MEMBER_ID_BUTTON.into(), button::spawn);
	//registry.insert(plygui_api::members::MEMBER_ID_LAYOUT_LINEAR.into(), layout_linear::spawn);
	registry.register_member(plygui_api::markup::MEMBER_TYPE_BUTTON.into(), button::spawn);
	registry.register_member(plygui_api::markup::MEMBER_TYPE_LINEAR_LAYOUT.into(), layout_linear::spawn);
}*/