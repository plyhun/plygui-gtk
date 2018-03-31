extern crate cc;
extern crate pkg_config;

use std::process::Command;

fn main() {
	let mut libs = unsafe {
		String::from_utf8_unchecked(
			Command::new("pkg-config")
			.args(&["--cflags", "gtk+-3.0", "glib-2.0"])
			.output()
			.unwrap()
			.stdout)};
	let len = libs.len();
	libs.split_off(len-1);
	let libs = libs.split(" ")
		.filter(|i| i.len()>2 && "-I" == &i[..2])
		.map(|i| &i[2..])
		.collect::<Vec<_>>();
	
	
	let mut cc_build = cc::Build::new();
	
	for lib in libs {
		cc_build.include(lib);
	}
	
    cc_build
        .include("ffi")
        .define("STATIC_BUILD", None)
        .opt_level(3)
        .warnings(false)
        .file("ffi/reckless_fixed.c")
        .compile("gtk_reckless_fixed");
}