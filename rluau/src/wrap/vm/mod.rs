#![allow(non_snake_case)]
pub use luau_src::vm as raw; // Raw Api if you don't want the wrappers.

mod types;
pub use types::*;

use super::prelude::*;

pub struct LuaError(i32);

#[derive(Debug, Error)]
pub enum LoadError {
	#[error("Failed to convert &str to proper null terminated string (*const char)")]
	ConversionError(std::ffi::NulError),
	#[error("Runtime lua error. See https://github.com/Roblox/luau/blob/f1649a43cdf7dd9e41496913672e82b30c067234/VM/include/lua.h#L26")]
	LuaError(i32), // TODO: Should map to crate::top::LuaStatus later.
}

impl Lua {
	#[inline(always)]
	pub fn new() -> Lua {
		Self(unsafe { raw::luaL_newstate() })
	}

	/// Tries to run a lua function on top of the stack (minus ``nargs``).
	/// # Parameters
	/// * `nargs` - Number of arguments to pop from the stack and push to the function
	/// * `nresults` - Number of results to push to the stack after a successful run
	/// * `errfunc` - Error handler
	/// # Returns
	/// Whether the function ran successfully
	pub fn pcall(
		&self,
		nargs: i32,
		nresults: i32,
		errfunc: i32,
	) -> Result<(), (i32, &'static str)> {
		// TODO: Validate parameters exist here
		let res = unsafe { raw::lua_pcall(self.0, nargs, nresults, errfunc) };

		match res {
			raw::LUA_OK => Ok(()), // OK
			status => {
				let err = match status {
					1 => Ok("Thread exited unexpectedly"),
					_ => unsafe {
						// May want to use the len argument to construct something else.
						let raw_str = raw::luaL_tolstring(self.0, -1, std::ptr::null_mut());
						CStr::from_ptr(raw_str).to_str()
					},
				};
				Err((status, err.unwrap_or("Unknown")))
			}
		}
	}

	/// Loads luau code into a lua function
	/// # Parameters
	/// * `chunk_name` - Name of the chunk as used in error messages and debugging.
	/// * `bytecode` - Bytecode retrieved from [crate::compiler::luau_compile].
	/// * `env` - Optional fenv to be retrieved from the stack.
	pub fn load<S: AsRef<[u8]>>(
		&self,
		chunk_name: &str,
		bytecode: S,
		env: Option<c_int>,
	) -> Result<(), LoadError> {
		let chunk_name = CString::new(chunk_name).map_err(LoadError::ConversionError)?;

		let result = unsafe {
			let bc = bytecode.as_ref();
			raw::luau_load(
				self.0,
				chunk_name.as_ptr(),
				bc.as_ptr() as _,
				bc.len(),
				env.unwrap_or(0),
			)
		};

		match result {
			raw::LUA_OK => Ok(()), // OK
			other => Err(LoadError::LuaError(other)),
		}
	}

	pub fn checkoption(&self, arg: i32, def: &str, lst: &[&str]) -> Result<i32, NulError> {
		let def = CString::new(def)?;

		let lst: Vec<CString> = lst.iter().map(|&s| CString::new(s).unwrap()).collect();
		let lst_ptrs: Vec<*const i8> = lst.iter().map(|s| s.as_ptr()).collect();

		Ok(unsafe { raw::luaL_checkoption(self.0, arg, def.as_ptr(), lst_ptrs.as_ptr()) })
	}
}
