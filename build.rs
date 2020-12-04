
fn main() {
	println!("cargo:rerun-if-changed=wrapper.h");

	let bindings = bindgen::Builder::default()
		.header("wrapper.h")
		.raw_line("#![allow(non_upper_case_globals)]")
		.raw_line("#![allow(non_camel_case_types)]")
		.raw_line("#![allow(non_snake_case)]")
		.raw_line("#![allow(clippy::all)]")
		.prepend_enum_name(false)
		.clang_args(&["-x", "c++", "-std=c++14"])
		.generate()
		.expect("Unable to generate bindings");
	bindings
		.write_to_file("src/bindings.rs")
		.expect("Could not write to src/bindings.rs");
	
	
	let mut build = cc::Build::new();
	build.cpp(true);
	build.define("CIMGUI_NO_EXPORT",None);
	build.define("IMGUI_DISABLE_WIN32_FUNCTIONS", None);
	build.define("IMGUI_DISABLE_OSX_FUNCTIONS", None);
	
	build.file("third-party/imgui/imgui.cpp");
	build.file("third-party/imgui/imgui_draw.cpp");
	build.file("third-party/imgui/imgui_widgets.cpp");
	build.file("third-party/imgui/imgui_demo.cpp");
	
	build.compile("libimgui.a");
}