use std::path::PathBuf;

macro_rules! luau_path {
	($l:literal) => {
		concat!("luau/", $l)
	};
}

static INCLUDES: &[&str] = &[
	luau_path!("VM/include"),
	luau_path!("Compiler/include"),
	luau_path!("Ast/include"),
];

static SOURCES: &[&str] = &[
	luau_path!("VM/src"),
	luau_path!("Compiler/src"),
	luau_path!("Ast/src"),
];

fn setup_configs(conf: &mut cc::Build) -> Result<(), std::io::Error> {
	conf.cpp(true)
		.flag_if_supported("-std=c++17")
		.flag_if_supported("/std:c++17")
		.includes(INCLUDES);

	for src in SOURCES {
		std::fs::read_dir(src)?
			.filter_map(|e| e.ok())
			.filter(|e| e.path().extension().map(|e| e == "cpp").unwrap_or(false))
			.for_each(|e| {
				conf.file(e.path());
			});
	}

	Ok(())
}

#[cfg(feature = "link")]
fn link(build: &mut cc::Build) -> Result<(), std::io::Error> {
	// Generate lib and object files
	setup_configs(build)?;
	build.compile("luau");

	// Link
	println!("cargo:rustc-link-lib=static=luau");
	Ok(())
}

#[cfg(not(feature = "link"))]
fn link() -> Result<(), std::io::Error> {
	Ok(())
}

macro_rules! cpp_bindings {
	() => {
		bindgen::builder()
			.clang_arg("-xc++")
			.clang_arg("-std=c++17")
			.layout_tests(false)
			.allowlist_type("(LUA|lua)(u|U)?.*")
			.allowlist_function("(LUA|lua)(u|U)?.*")
			.allowlist_var("(LUA|lua)(u|U)?.*")
			.prepend_enum_name(false)
			.size_t_is_usize(true)
			.c_naming(false)
			.disable_name_namespacing()
	};
}

fn main() -> Result<(), std::io::Error> {
	let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());

	// VM Functions
	cpp_bindings!()
		.header(luau_path!("VM/include/lua.h"))
		.header(luau_path!("VM/include/lualib.h"))
		.blocklist_function("lua_pushvfstring")
		.blocklist_type("va_list")
		.generate()
		.expect("Failed to generate VM bindings")
		.write_to_file(out_dir.join("luau_vm.rs"))?;

	// VM Configs
	cpp_bindings!()
		.header(luau_path!("VM/include/luaconf.h"))
		.generate()
		.expect("Failed to generate VM bindings")
		.write_to_file(out_dir.join("luau_vm_conf.rs"))?;

	// Compiler bindings
	cpp_bindings!()
		.allowlist_var("(LUA|lua)(u|U)?.*")
		.header(luau_path!("Compiler/include/luacode.h"))
		.derive_default(true)
		.derive_copy(true)
		.derive_partialeq(true)
		.derive_eq(true)
		.derive_hash(true)
		.generate()
		.expect("Failed to generate Compiler bindings")
		.write_to_file(out_dir.join("luau_compiler.rs"))?;

	let mut b = cc::Build::new();
	link(&mut b)?;

	println!("cargo:rerun-if-changed=src/lib.rs");
	println!("cargo:rerun-if-changed=src/raw/mod.rs");

	Ok(())
}
