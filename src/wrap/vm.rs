#![allow(non_snake_case)]
pub use crate::raw::vm::*;
pub use crate::raw::vm as raw; // Raw Api if you don't want the wrappers.

use super::prelude::*;
use crate::raw::vm;

#[derive(Debug, Error)]
pub enum LoadError {
	#[error("Failed to convert &str to proper null terminated string (*const char)")]
	ConversionError(std::ffi::NulError),
	#[error("Runtime lua error. See https://github.com/Roblox/luau/blob/f1649a43cdf7dd9e41496913672e82b30c067234/VM/include/lua.h#L26")]
	LuaError(i32) // TODO: Should map to crate::top::LuaStatus later.
}

pub fn luau_load(state: *mut lua_State, chunk_name: &str, bytecode: &str, size: u64, env: i32) -> Result<(), LoadError> {
	let chunk_name = CString::new(chunk_name).map_err(LoadError::ConversionError)?;
	let bytecode = CString::new(bytecode).map_err(LoadError::ConversionError)?;

	let result = unsafe { vm::luau_load(state, chunk_name.as_ptr(), bytecode.as_ptr(), size, env) };

	match result {
		0 => Ok(()), // OK
		other => Err( LoadError::LuaError(other) )
	}
}

pub fn luaL_checkoption(state: *mut lua_State, arg: i32, def: &str, lst: &[&str]) -> Result<i32, NulError> {
	let def = CString::new(def)?;

	let lst: Vec<CString> = lst.iter().map(|&s| CString::new(s).unwrap()).collect();
	let lst_ptrs: Vec<*const i8> = lst.iter().map(|s| s.as_ptr()).collect();

	Ok( unsafe { vm::luaL_checkoption(state, arg, def.as_ptr(), lst_ptrs.as_ptr()) } )
}