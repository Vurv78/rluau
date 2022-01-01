use rluau::{
	compiler::{self, CompileError, CompileOptions},
	vm,
};

fn main() -> Result<(), CompileError> {
	let mut opts = CompileOptions::default();
	let bc = compiler::compile("print('hello luau!')", &mut opts)?;

	let lua = vm::Lua::new();

	lua.load("main", bc, None).expect("Failed to load bytecode");

	let result = lua.pcall(0, 0, 0);

	println!("Result: {:?}", result);
	// Error, because standard library is not loaded.

	Ok(())
}
