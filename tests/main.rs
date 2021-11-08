use cxx::let_cxx_string;
use luau_src::{ c_int };

#[test]
fn test_main() {
	// This will not compile until we generate Compile/ParseOptions as POD (https://github.com/google/autocxx/issues/667 needs to be fixed)
	use luau_src::compiler::Luau as compiler;
	let_cxx_string!(s = "print('Hello luau!')");

	let options = compiler::CompileOptions {
		bytecodeVersion: c_int(1),
		optimizationLevel: c_int(1),
		debugLevel: c_int(1),
		coverageLevel: c_int(0),
		vectorLib: std::ptr::null(),
		vectorCtor: std::ptr::null()
	};

	let parse_opts = compiler::ParseOptions {
		allowTypeAnnotations: true,
		supportContinueStatement: true,
		allowDeclarationSyntax: true,
		captureComments: true
	};

	let bytecode = unsafe {
		compiler::compile(
			&s,
			&compile_opts,
			&parse_opts,
			std::ptr::null_mut()
		)
	};
}