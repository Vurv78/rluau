use rluau::{
	compiler::{self, CompileError, CompileOptions},
	vm::{self, LuauOptions},
};

const SOURCE: &str = r#"
local foo: number = 55;
exported = { }
print( exported, foo);
"#;

fn main() -> Result<(), rluau::Error> {
	let mut opts = CompileOptions::default();
	let bc = compiler::compile(SOURCE, &mut opts)?;

	let luau = vm::Luau::new();

	let var = luau.get_global("exported");
	println!("{:?}", var); // Nil

	luau.load("main", bc, None)?;
	let result = luau.pcall(0, 0, 0);
	println!("Result: {:?}", result); // OK

	let var = luau.get_global("exported");
	println!("{:?}", var); // Table

	Ok(())
}
