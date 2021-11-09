# 🌔 ``luau-rs``
> [Luau](https://github.com/Roblox/luau) bindings for the [Rust](https://www.rust-lang.org) programming language using [bindgen](https://github.com/rust-lang/rust-bindgen)

## ⚠️ Disclaimer
This does not provide bindings for everything as luau does not provide an adequate API for C bindings, which trips up bindgen & makes ffi exponentially more difficult (thanks to using C++'s ``std::string`` and whatnot). See [luau/121](https://github.com/Roblox/luau/issues/121). (It is also *untested* thanks to this..)

## Usage
Add this to your ``Cargo.toml``
```toml
[dependencies]
luau = { version = "0.2.0", package = "luau-src" }
```

## Example
```rust
// See this in action in the /tests/
use luau_src::{ c_int, c_ulonglong, compiler::Luau::{CompileOptions, ParseOptions, compile}, vm };
unsafe fn example() {
	let_cxx_string!(s = "local a: number = 5; print(a)");

	let compile_opts = CompileOptions::default();
	let parse_opts = ParseOptions::default();

	let bytecode = compile( &s, &compile_opts, &parse_opts, std::ptr::null_mut() );

	let state = vm::luaL_newstate();
	let int = vm::luau_load(state, b"main\0".as_ptr() as *const i8, bytecode.as_ptr() as *const i8, c_ulonglong(bytecode.len() as u64), c_int(0));
	assert_eq!(int.0, 0, "Failed to load bytecode");

	vm::luaL_openlibs(state);
	let status = vm::lua_pcall(state, c_int(0), c_int(0), c_int(0)).0;

	if status != 0 {
		let err = match status {
			1 => Ok("Thread exited unexpectedly"),
			_ => {
				let raw_str = vm::luaL_tolstring(state, c_int(-1), &mut c_ulonglong(0) as *mut c_ulonglong);
				std::ffi::CStr::from_ptr(raw_str).to_str()
			}
		};
		let err = err.unwrap_or("Unknown error");
		let raw_str = vm::lua_debugtrace(state);
		let debug_trace = std::ffi::CStr::from_ptr(raw_str).to_str().unwrap_or("Couldn't convert to &str");
		println!("{} Stack backtrace:\n {}", err, debug_trace);
	}
}
```

## Requirements
See the requirements for bindgen [here](https://rust-lang.github.io/rust-bindgen/requirements.html)