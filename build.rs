use std::path::{Path, PathBuf};

#[derive(Debug)]
enum GeneratorError {
	BindgenFailure,
	WriteFailure( std::io::Error ),
	VarError( std::env::VarError ),
}

const SOURCES: [&str; 4] = ["luau/VM/src", "luau/Ast/src", "luau/Analysis/src", "luau/Compiler/src"];
const INCLUDES: [&str; 4] = ["luau/VM/include", "luau/Ast/include", "luau/Analysis/include", "luau/Compiler/include"];

fn setup_configs(conf: &mut cc::Build) -> Result<(), std::io::Error> {
	conf
		.opt_level(2)
		.cpp(true)
		.flag_if_supported("-std=c++17")
		.flag_if_supported("/std:c++17")
		.includes(INCLUDES.iter())
		.out_dir( Path::new("./build") );

	for src in SOURCES {
		let iter = walkdir::WalkDir::new(src).into_iter().filter_map(|e| {
			match e {
				Ok(e) => {
					let ext = e.path().extension();
					if let Some(ext) = ext {
						match ext.to_str() {
							Some("cpp") => Some(e.into_path()),
							_ => None
						}
					} else {
						None
					}
				},
				_ => None
			}
		});
		conf.files(iter);
	}

	Ok(())
}

fn gen_bindings<S: Into<String>, P: AsRef<Path>>(src: S, out: P) -> Result<(), GeneratorError> {
	let bindings = bindgen::builder()
		.enable_cxx_namespaces()
		.clang_arg("-std=c++17")
		.clang_arg("-xc++")
		.clang_args( INCLUDES.map(|s| format!("-I{}", s)) )
		.allowlist_recursively(true)
		.allowlist_function("(lua|Lua).*")
		.allowlist_var("lua.*")
		// Allowing for Lua causes for a variable that I can't seem to be able to blocklist to appear.
		// This causes https://github.com/rust-lang/rust-bindgen/issues/1496.
		.allowlist_type("(lua|Lua).*")
		.allowlist_type(".*(Luau|BytecodeBuilder).*")
		.header(src)
		.conservative_inline_namespaces()
		.layout_tests(false)
		.generate()
		.map_err(|_| GeneratorError::BindgenFailure)?;

	bindings
		.write_to_file(out)
		.map_err(GeneratorError::WriteFailure)?;

	Ok(())
}

#[cfg(not(feature = "no-link"))]
fn link() -> Result<(), std::io::Error> {
	let mut conf = cc::Build::new();
	setup_configs(&mut conf)?;
	conf.compile("luau");

	println!("cargo:rustc-link-lib=static=luau");
}

#[cfg(feature = "no-link")]
fn link() -> Result<(), std::io::Error> { Ok(()) }

fn main() -> Result<(), GeneratorError> {
	// Generate .lib and object files
	link().map_err(GeneratorError::WriteFailure)?;

	let out_dir = PathBuf::from( std::env::var("OUT_DIR").map_err(GeneratorError::VarError)? );

	gen_bindings("luau/Compiler/include/Luau/Compiler.h", out_dir.join("binds_compiler.rs"))?;
	gen_bindings("luau/Ast/include/Luau/Ast.h", out_dir.join("binds_ast.rs"))?;
	gen_bindings("luau/VM/include/lua.h", out_dir.join("binds_vm.rs"))?;

	Ok(())
}