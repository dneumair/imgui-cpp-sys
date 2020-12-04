use std::path::Path;
use std::fs::File;
use std::io::Write;
use bindgen::Bindings;

fn main() {
	println!("cargo:warning=Generating bindings");
	let bindings = bindgen::Builder::default()
		.header("wrapper.h")
		.prepend_enum_name(false)
		.clang_args(&["-x", "c++", "-std=c++14"])
		.generate()
		.expect("Unable to generate bindings");
	
	
	let path = Path::new("src/bindings.rs");
	let mut out_file = File::create(path).expect("Could not create src/bindings.rs");
	out_file.write_all("
	#![allow(non_upper_case_globals)]\n
	#![allow(non_camel_case_types)]\n
	#![allow(non_snake_case)]\n
	".as_bytes());
	
	out_file.write(
	bindings.to_string().as_bytes()
	);
	
	drop(out_file);
}