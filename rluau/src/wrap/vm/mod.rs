pub(crate) mod types;
pub use types::{Luau, LuauOptions};

mod api;
mod value;

pub(crate) mod prelude {
	pub use luau_src::vm as raw;
	pub use super::types::{ Luau, String as LuauString, LightUserdata as LuauLightUserdata, Function as LuauFunction, LuauRef };

	pub use super::value::Value;

	pub use super::super::prelude::*;
}

use prelude::*;

pub struct LuaError(i32);

#[derive(Debug, Error)]
pub enum LoadError {
	#[error("Failed to convert &str to proper null terminated string (*const char)")]
	ConversionError(std::ffi::NulError),
	#[error("Runtime lua error. See https://github.com/Roblox/luau/blob/f1649a43cdf7dd9e41496913672e82b30c067234/VM/include/lua.h#L26")]
	LuaError(i32), // TODO: Should map to crate::top::LuaStatus later.
}

impl Luau {
	pub fn new() -> Self {
		Self::new_with_options( &LuauOptions::default() )
	}

	#[cfg(feature = "bitflags")]
	pub fn new_with_options(opts: &LuauOptions) -> Self {
		use self::types::StdLib;
		// TODO: Should use lua_newstate rather than luaL_newstate for control over error handler and whatnot.

		let state = Self::__new(unsafe { raw::luaL_newstate() });

		unsafe {
			if opts.libs.contains(StdLib::BASE) {
				raw::luaopen_base(state.raw);
			}

			if opts.libs.contains(StdLib::COROUTINE) {
				raw::luaopen_coroutine(state.raw);
			}

			if opts.libs.contains(StdLib::TABLE) {
				raw::luaopen_table(state.raw);
			}

			if opts.libs.contains(StdLib::OS) {
				raw::luaopen_os(state.raw);
			}

			if opts.libs.contains(StdLib::STRING) {
				raw::luaopen_string(state.raw);
			}

			if opts.libs.contains(StdLib::BIT32) {
				raw::luaopen_bit32(state.raw);
			}

			if opts.libs.contains(StdLib::UTF8) {
				raw::luaopen_utf8(state.raw);
			}

			if opts.libs.contains(StdLib::MATH) {
				raw::luaopen_math(state.raw);
			}

			if opts.libs.contains(StdLib::DEBUG) {
				raw::luaopen_debug(state.raw);
			}
		}

		state
	}
}
