use std::path::{Path, PathBuf};

const INCLUDES: [&str; 5] = ["vendor/luau/VM/include", "vendor/luau/Ast/include", "vendor/luau/Analysis/include", "vendor/luau/Compiler/include", "vendor/luau/Analysis/include"];

fn setup_configs(conf: &mut cc::Build) -> Result<(), std::io::Error> {
	const SOURCES: [&str; 5] = ["vendor/luau/VM/src", "vendor/luau/Ast/src", "vendor/luau/Analysis/src", "vendor/luau/Compiler/src", "vendor/luau/Analysis/src"];

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

fn main() -> Result<(), std::io::Error> {
	// Bindgen + CXX using Autocxx

	let b = autocxx_build::Builder::new("src/raw/mod.rs", INCLUDES)
		.extra_clang_args(&["-std=c++17"])
		.expect_build();

	link(b)?;
	println!("cargo:rerun-if-changed=src/lib.rs");

	Ok(())
}