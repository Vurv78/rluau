use luau_src::vm as raw;

#[repr(transparent)]
pub struct Lua(pub *mut raw::lua_State);

impl Drop for Lua {
	fn drop(&mut self) {
		unsafe { raw::lua_close(self.0) }
	}
}

impl Default for Lua {
	fn default() -> Self {
		Self::new()
	}
}
