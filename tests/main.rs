use cxx::let_cxx_string;
use luau_src::{
	c_int, c_ulonglong,
	compiler::{self, Luau::{CompileOptions, ParseOptions}},
	vm
};
use std::ffi::CStr;

const SAMPLE_TYPE_NUMBER: &str = "local a: number = 5";

#[test]
fn test_compile_and_run_thread_raw() {
	use vm::raw as vm;
	use compiler::raw as compiler;

	let_cxx_string!(s = SAMPLE_TYPE_NUMBER);

	let compile_opts = CompileOptions::default();
	let parse_opts = ParseOptions::default();

	let bytecode = unsafe {
		compiler::Luau::compile(
			&s,
			&compile_opts,
			&parse_opts,
			std::ptr::null_mut()
		)
	};

	let state = vm::luaL_newstate();
	unsafe {
		let int = vm::luau_load(state, b"main\0".as_ptr() as *const i8, bytecode.as_ptr() as *const i8, bytecode.len() as u64, 0);
		assert_eq!(int, 0, "Failed to load bytecode");

		let thread = vm::lua_newthread(state);
		vm::lua_pushvalue(state, c_int(-2));
		vm::lua_remove(state, c_int(-3));
		vm::lua_xmove(state, thread, c_int(1));

		vm::luaL_openlibs(thread);
		let status = vm::lua_resume(thread, std::ptr::null_mut(), c_int(0)).0;

		if status != 0 {
			let err = match status {
				1 => Ok("Thread exited unexpectedly"),
				_ => {
					let raw_str = vm::luaL_tolstring(thread, c_int(-1), &mut c_ulonglong(0) as *mut c_ulonglong);
					CStr::from_ptr(raw_str).to_str()
				}
			};
			let err = err.unwrap_or("Unknown error");
			let raw_str = vm::lua_debugtrace(thread);
			let debug_trace = CStr::from_ptr(raw_str).to_str().unwrap_or("Couldn't convert to &str");
			panic!("{} Stack backtrace:\n {}", err, debug_trace);
		}
	}
}

#[test]
fn test_compile_and_run_raw() {
	use vm::raw as vm;
	use compiler::raw as compiler;

	let_cxx_string!(s = SAMPLE_TYPE_NUMBER);

	let compile_opts = CompileOptions::default();
	let parse_opts = ParseOptions::default();

	let bytecode = unsafe {
		compiler::Luau::compile(
			&s,
			&compile_opts,
			&parse_opts,
			std::ptr::null_mut()
		)
	};

	let state = vm::luaL_newstate();
	unsafe {
		let int = vm::luau_load(state, b"main\0".as_ptr() as *const i8, bytecode.as_ptr() as *const i8, bytecode.len() as u64, 0);
		assert_eq!(int, 0, "Failed to load bytecode, {}", int);

		vm::luaL_openlibs(state);
		let status = vm::lua_pcall(state, c_int(0), c_int(0), c_int(0)).0;

		if status != 0 {
			let err = match status {
				1 => Ok("Thread exited unexpectedly"),
				_ => {
					let raw_str = vm::luaL_tolstring(state, c_int(-1), &mut c_ulonglong(0) as *mut c_ulonglong);
					CStr::from_ptr(raw_str).to_str()
				}
			};
			let err = err.unwrap_or("Unknown error");
			let raw_str = vm::lua_debugtrace(state);
			let debug_trace = CStr::from_ptr(raw_str).to_str().unwrap_or("Couldn't convert to &str");
			panic!("{} Stack backtrace:\n {}", err, debug_trace);
		}

	}
}

#[test]
fn test_compile_and_run() {
	let (compile, parse) = (CompileOptions::default(), ParseOptions::default());
	let bytecode = compiler::luau::compile(SAMPLE_TYPE_NUMBER, &compile, &parse, None);
	println!("bytecode [{}]", bytecode);
}