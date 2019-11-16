use cc;
use pkg_config;

fn main() {
    let gtk_probe = pkg_config::Config::new().atleast_version("3.0").probe("gtk+-3.0").unwrap();
    let glib_probe = pkg_config::Config::new().atleast_version("2.0").probe("glib-2.0").unwrap();

    let mut cc_build = cc::Build::new();

    for lib in gtk_probe.include_paths.as_slice() {
        cc_build.include(lib.to_str().unwrap());
    }
    for lib in glib_probe.include_paths.as_slice() {
        cc_build.include(lib.to_str().unwrap());
    }
    cc_build.include("ffi")
        .define("STATIC_BUILD", None)
        .opt_level(3)
        .warnings(false)
        .file("ffi/reckless_fixed.c")
        .file("ffi/reckless_button.c")
        .file("ffi/reckless_paned.c")
        .file("ffi/reckless_frame.c")
        .file("ffi/reckless_box.c")
        .file("ffi/reckless_text_view.c")
        .file("ffi/reckless_label.c")
        .file("ffi/reckless_progress_bar.c")
        .file("ffi/reckless_tree_view.c")
        .file("ffi/reckless_cell_renderer.c")
        .file("ffi/reckless_list_box.c")
        .compile("gtk_reckless");
}
