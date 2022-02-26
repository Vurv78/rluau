pub use std::os::raw::{c_char, c_int, c_void};

#[allow(non_camel_case_types)]
pub type c_size_t = usize;

pub use luau_src::vm::{lua_Number as LuauNumber, lua_Integer as LuauInteger};
pub use super::vm::types::{String as LuauString, LightUserdata as LuauLightUserdata, Function as LuauFunction};

use crate::compiler::CompileError;


// Generic error from luau
#[derive(Debug, thiserror::Error)]
pub enum LuauError {
	#[error("Error when converting into CString: `{0}`")]
	NulError(#[from] std::ffi::NulError),

	#[error("Error when parsing / lua: `{0}`")]
	Parsing(String),

	#[error("Hit lua memory limit: `{0}`")]
	Memory(String),

	#[error("Error during garbage collection: `{0}`")]
	GarbageCollection(String),

	#[error("Luau runtime error: `{0}`")]
	Runtime(String),

	#[error("Compile error: `{0}`")]
	Compile(#[from] CompileError),

	#[error("Unimplemented")]
	Todo
}

pub type LuauResult<T> = Result<T, LuauError>;