use crate::types::*;
use luau_src::compiler as raw;
use std::ffi::{CStr, CString};

mod types;
pub use types::*;

#[derive(Debug, thiserror::Error)]
pub enum CompileError {
	#[error("Failed to convert input code to CString {0}")]
	Conversion(#[from] std::ffi::NulError),
}

pub fn compile<S: AsRef<str>>(
	source: S,
	options: &mut CompileOptions,
) -> Result<&'static [u8], CompileError> {
	let source = source.as_ref();
	let source_cstring = CString::new(source)?;
	let bc_size: &mut c_size_t = &mut 0;

	// Luau will not deallocate the bytecode for you so we can return this as a 'static slice.
	let bc = unsafe {
		raw::luau_compile(
			source_cstring.as_ptr(),
			source.len(),
			&mut options.0,
			bc_size,
		)
	};
	let c_str = unsafe { CStr::from_ptr(bc) };
	Ok(c_str.to_bytes())
}
