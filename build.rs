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
		.header(src)
		.allowlist_function("(luau|Luau).*")
		.allowlist_var("(luau|Luau).*")
		.allowlist_type("(luau|Luau).*")
		.layout_tests(false)
		.generate()
		.map_err(|_| GeneratorError::BindgenFailure)?;

	bindings
		.write_to_file(out)
		.map_err(GeneratorError::WriteFailure)?;

	Ok(())
}

fn main() -> Result<(), GeneratorError> {

	// Generate .lib and object files
	let mut conf = cc::Build::new();
	setup_configs(&mut conf).map_err(GeneratorError::WriteFailure)?;
	conf.compile("luau");

	let out_dir = PathBuf::from( std::env::var("OUT_DIR").map_err(GeneratorError::VarError)? );

	gen_bindings("luau/Compiler/include/Luau/Compiler.h", out_dir.join("binds_compiler.rs"))?;
	gen_bindings("luau/Ast/include/Luau/Ast.h", out_dir.join("binds_ast.rs"))?;
	gen_bindings("luau/VM/include/lua.h", out_dir.join("binds_vm.rs"))?;

	Ok(())
}