#![feature(new_uninit)]

#[macro_use]
extern crate plygui_api;
#[macro_use]
extern crate lazy_static;

#[macro_use]
pub extern crate glib;
pub use gdk;
pub use gtk;
pub use libc;
pub use pango;

pub mod reckless;

#[macro_use]
pub mod common;

mod application;
mod button;
mod frame;
mod image;
mod layout_linear;
mod message;
mod splitted;
mod text;
mod tray;
mod window;
mod progress_bar;
mod list;
mod tree;
mod table;

default_markup_register_members!();
default_pub_use!();
