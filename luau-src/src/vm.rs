use std::os::raw::{c_int, c_char};

use crate::raw::vm as raw;
pub use raw::*;

#[inline(always)]
pub unsafe fn lua_pop(l: *mut lua_State, ind: c_int) {
	raw::lua_settop(l, -(ind) - 1);
}

#[inline(always)]
pub unsafe fn lua_getglobal(l: *mut lua_State, name: *const c_char) -> () {
	lua_getfield(l, raw::LUA_GLOBALSINDEX, name);
}

#[inline(always)]
pub unsafe fn lua_setglobal(l: *mut lua_State, name: *const c_char) -> () {
	lua_setfield(l, raw::LUA_GLOBALSINDEX, name);
}

#[inline(always)]
pub unsafe fn lua_tostring(l: *mut lua_State, i: c_int) -> *const c_char {
	raw::lua_tolstring(l, i, std::ptr::null_mut())
}

#[inline(always)]
pub unsafe fn lua_tonumber(l: *mut lua_State, i: c_int) -> lua_Number {
	raw::lua_tonumberx(l, i, std::ptr::null_mut())
}

#[inline(always)]
pub unsafe fn lua_tointeger(l: *mut lua_State, i: c_int) -> lua_Integer {
	raw::lua_tointegerx(l, i, std::ptr::null_mut())
}

#[inline(always)]
pub unsafe fn lua_tounsigned(l: *mut lua_State, i: c_int) -> lua_Unsigned {
	raw::lua_tounsignedx(l, i, std::ptr::null_mut())
}