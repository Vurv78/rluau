use crate::compiler::CompileError;

use super::prelude::*;

impl Luau {
	// Maybe these should be exposed but not sure about their names..
	fn pop(&self, ind: i32) {
		unsafe { raw::lua_pop(self.raw, ind) };
	}

	fn get_type(&self, ind: i32) -> i32 {
		unsafe { raw::lua_type(self.raw, ind) }
	}

	fn to_string(&self, ind: i32) -> Option<String> {
		let mut len = 0;
		let ptr = unsafe { raw::lua_tolstring(self.raw, ind, &mut len) };
		if ptr.is_null() {
			return None
		}
		Some(unsafe { CStr::from_ptr(ptr).to_string_lossy().into_owned() })
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
	) -> LuauResult<()> {
		let chunk_name = CString::new(chunk_name)?;

		let result = unsafe {
			let bc = bytecode.as_ref();
			raw::luau_load(
				self.raw,
				chunk_name.as_ptr(),
				bc.as_ptr() as _,
				bc.len(),
				env.unwrap_or(0),
			)
		};

		match self.get_type(-1) {
			raw::LUA_TFUNCTION => (),
			raw::LUA_TSTRING => {
				let reason = self.to_string(-1).unwrap();
				return Err( LuauError::Compile( CompileError::Syntax(reason) ) )
			},
			// Shouldn't be possible?
			_ => return Err( LuauError::Todo ),
		};

		if let raw::LUA_OK | raw::LUA_YIELD = result {
			return Ok(());
		}

		let err = unsafe { raw::lua_tostring(self.raw, -1) };
		let err = unsafe { std::ffi::CStr::from_ptr(err) };
		let err = err.to_string_lossy().to_string();
		unsafe { raw::lua_pop(self.raw, 1) };

		Err(match result {
			raw::LUA_ERRSYNTAX => LuauError::Parsing(err),
			raw::LUA_ERRRUN => LuauError::Runtime(err),
			raw::LUA_ERRMEM => LuauError::Memory(err),
			raw::LUA_ERRERR => LuauError::Runtime(err),
			_ => panic!("Unknown error code: {}", result),
		})
	}

	/// Tries to run a lua function on top of the stack (minus ``nargs``).
	/// # Parameters
	/// * `nargs` - Number of arguments to pop from the stack and push to the function
	/// * `nresults` - Number of results to push to the stack after a successful run
	/// * `errfunc` - Error handler
	/// # Returns
	/// Whether the function ran successfully, else tuple containing the error code and message.
	pub fn pcall(
		&self,
		nargs: i32,
		nresults: i32,
		errfunc: i32,
	) -> Result<(), (i32, &'static str)> {
		let top = unsafe { raw::lua_gettop(self.raw) };
		if top < nargs {
			return Err((raw::LUA_ERRRUN, "Stack underflow"));
		}

		match unsafe { raw::lua_pcall(self.raw, nargs, nresults, errfunc) } {
			raw::LUA_OK => Ok(()), // OK
			status => {
				let err = match status {
					1 => Ok("Thread exited unexpectedly"),
					_ => unsafe {
						// May want to use the len argument to construct something else.
						let raw_str = raw::luaL_tolstring(self.raw, -1, std::ptr::null_mut());
						CStr::from_ptr(raw_str).to_str()
					},
				};
				Err((status, err.unwrap_or("Unknown")))
			}
		}
	}

	pub fn checkoption(&self, arg: i32, def: &str, lst: &[&str]) -> LuauResult<i32> {
		let def = CString::new(def)?;

		let lst: Vec<CString> = lst.iter().map(|&s| CString::new(s).unwrap()).collect();
		let lst_ptrs: Vec<*const i8> = lst.iter().map(|s| s.as_ptr()).collect();

		Ok(unsafe { raw::luaL_checkoption(self.raw, arg, def.as_ptr(), lst_ptrs.as_ptr()) })
	}

	pub fn get_global<'luau>(&self, name: &str) -> LuauResult<Value<'luau>> {
		let name = CString::new(name)?;

		unsafe { raw::lua_getglobal(self.raw, name.as_ptr()) };
		let ty = unsafe { raw::lua_type(self.raw, -1) };
		self.pop(1);

		Ok(match ty {
			raw::LUA_TBOOLEAN => Value::Boolean(unsafe { raw::lua_toboolean(self.raw, -1) != 0 }),
			raw::LUA_TSTRING => Value::String(unsafe { raw::lua_tostring(self.raw, -1) }.into()),
			raw::LUA_TNUMBER => Value::Number(unsafe { raw::lua_tonumber(self.raw, -1) }),
			raw::LUA_TTABLE => Value::Table(()),
			raw::LUA_TLIGHTUSERDATA => Value::LightUserData( LuauLightUserdata(unsafe { raw::lua_touserdata(self.raw, -1) })),
			raw::LUA_TFUNCTION => Value::Function(unsafe { raw::lua_tocfunction(self.raw, -1) }.into()),
			raw::LUA_TNIL => Value::Nil,
			_ => return Err( LuauError::Todo ),
		})
	}
}