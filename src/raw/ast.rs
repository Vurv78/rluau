pub use super::i_ast::*;

#[cxx::bridge]
mod ffi {
	extern "C++" {}
}

pub use ffi::*;