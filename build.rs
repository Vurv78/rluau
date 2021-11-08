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
		.includes(INCLUDES);

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

#[cfg(not(feature = "no-link"))]
fn link(mut build: cc::Build) -> Result<(), std::io::Error> {
	// Generate lib and object files
	setup_configs(&mut build)?;
	build.compile("luau");

	// Link
	println!("cargo:rustc-link-lib=static=luau");
	Ok(())
}

#[cfg(feature = "no-link")]
fn link() -> Result<(), std::io::Error> { Ok(()) }

fn main() -> Result<(), GeneratorError> {
	// Bindgen + CXX using Autocxx
	let b = autocxx_build::Builder::new("src/lib.rs", INCLUDES)
		.extra_clang_args(&["-std=c++17"])
		.expect_build();

	link(b).map_err(GeneratorError::WriteFailure)?;
	println!("cargo:rerun-if-changed=src/lib.rs");

	Ok(())
}