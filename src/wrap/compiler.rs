pub use crate::raw::compiler as raw; // Raw Api if you don't want the wrappers.
pub use raw::*;

use super::prelude::*;

pub mod luau {
	pub use super::Luau::{self, *};

	pub fn compile<S: AsRef<str>>(source: S, opts: &CompileOptions, parse_opts: &ParseOptions, encoder: Option<*mut BytecodeEncoder>) -> String {
		cxx::let_cxx_string!(s = source.as_ref());
		// C++ gives the string by value but we can't own C++ strings so let's just copy it.
		unsafe { Luau::compile(&s, opts, parse_opts, encoder.unwrap_or(std::ptr::null_mut())) }.to_string()
	}
}

impl Default for Luau::CompileOptions {
	fn default() -> Self {
		Self {
			bytecodeVersion: 1,
			optimizationLevel: 1,
			debugLevel: 1,
			coverageLevel: 0,
			vectorLib: std::ptr::null(),
			vectorCtor: std::ptr::null()
		}
	}
}

impl Default for Luau::ParseOptions {
	fn default() -> Self {
		Self {
			allowTypeAnnotations: true,
			supportContinueStatement: true,
			allowDeclarationSyntax: true,
			captureComments: true
		}
	}
}