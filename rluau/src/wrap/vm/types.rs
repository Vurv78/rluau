use luau_src::vm as raw;
use std::{borrow::Cow, marker::PhantomData, cell::Cell};
pub use std::os::raw::{c_char, c_int, c_void};

pub struct Luau {
	pub raw: *mut raw::lua_State,
	// Use OnceCell?
	pub storage: *mut raw::lua_State
}

impl Luau {
	pub(crate) fn __new(raw: *mut raw::lua_State) -> Self {
		Self {
			raw,
			storage: unsafe { raw::luaL_newstate() }
		}
	}
}

#[cfg(feature = "bitflags")]
bitflags::bitflags! {
	pub struct StdLib: u32 {
		const BASE = 1;
		const COROUTINE = 2;
		const TABLE = 4;
		const OS = 8;
		const STRING = 16;
		const BIT32 = 32;
		const UTF8 = 64;
		const MATH = 128;
		const DEBUG = 256;

		const ALL = StdLib::BASE.bits | StdLib::COROUTINE.bits | StdLib::TABLE.bits | StdLib::OS.bits | StdLib::STRING.bits | StdLib::BIT32.bits | StdLib::UTF8.bits | StdLib::MATH.bits | StdLib::DEBUG.bits;
		const ALL_NO_DEBUG = StdLib::BASE.bits | StdLib::COROUTINE.bits | StdLib::TABLE.bits | StdLib::OS.bits | StdLib::STRING.bits | StdLib::BIT32.bits | StdLib::UTF8.bits | StdLib::MATH.bits;
		const DEFAULT = StdLib::BASE.bits | StdLib::TABLE.bits | StdLib::STRING.bits | StdLib::MATH.bits;
	}
}

#[non_exhaustive]
pub struct LuauOptions {
	#[cfg(feature = "bitflags")]
	/// Default standard libraries to import.
	pub libs: StdLib
}

impl Default for LuauOptions {
	fn default() -> Self {
		Self {
			#[cfg(feature = "bitflags")]
			libs: StdLib::DEFAULT
		}
	}
}

impl Drop for Luau {
	fn drop(&mut self) {
		unsafe { raw::lua_close(self.raw) }
	}
}

impl Default for Luau {
	fn default() -> Self {
		Self::new()
	}
}

#[allow(unused)]
pub struct LuauRef {
	state: *mut raw::lua_State,
	idx: i32
}


#[repr(transparent)]
#[derive(Debug)]
pub struct LightUserdata(pub *mut c_void);

#[repr(transparent)]
#[derive(Debug)]
pub struct String<'luau>(Cow<'luau, str>);

impl<'luau> From<*const c_char> for String<'luau> {
	// This should be removed entirely :/
	#[allow(clippy::not_unsafe_ptr_arg_deref)]
	fn from(value: *const c_char) -> Self {
		if value.is_null() {
			String( Cow::Owned(std::string::String::new()) )
		} else {
			String(unsafe {
				std::ffi::CStr::from_ptr(value).to_string_lossy()
			})
		}
	}
}

use luau_src::vm::lua_CFunction;
#[repr(transparent)]
#[derive(Debug)]
pub struct Function<'luau> {
	pub inner: lua_CFunction,
	// Placeholder for a potential lua ref system as rlua handles lua.
	__placeholder: PhantomData<Cell<&'luau()>>
}

impl<'luau> From<lua_CFunction> for Function<'luau> {
	fn from(value: lua_CFunction) -> Self {
		Self {
			inner: value,
			__placeholder: PhantomData
		}
	}
}
