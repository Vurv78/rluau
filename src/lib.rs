use autocxx::include_cpp;

include_cpp! {
	#include "Luau/Compiler.h"

	name!(comp_inner)
	safety!(unsafe)

	// This should be generate_pod! when https://github.com/google/autocxx/issues/667 is fixed
	generate!("Luau::CompileOptions")
	generate!("Luau::CompileError")
	generate!("Luau::compileOrThrow")
	generate!("Luau::compile")
}

pub mod compiler {
	pub use crate::comp_inner::*;
}

include_cpp! {
	#include "Luau/Ast.h"

	name!(ast_inner)
	safety!(unsafe)

	generate!("Luau::AstVisitor")
	generate!("Luau::AstNode")
	generate!("Luau::AstExpr")
	generate!("Luau::AstExprGroup")
	generate!("Luau::AstExprConstantNil")
	generate!("Luau::AstExprConstantBool")
	generate!("Luau::AstExprConstantNumber")
	generate!("Luau::AstExprConstantString")
	generate!("Luau::AstExprLocal")
	generate!("Luau::AstExprGlobal")
	generate!("Luau::AstExprVarargs")
	generate!("Luau::AstExprCall")
	generate!("Luau::AstExprIndexName")
	generate!("Luau::AstExprIndexExpr")
	generate!("Luau::AstExprFunction")
	generate!("Luau::AstExprTable")
	generate!("Luau::AstExprUnary")
	generate!("Luau::AstExprBinary")
	generate!("Luau::AstExprTypeAssertion")
	generate!("Luau::AstExprIfElse")
	generate!("Luau::AstExprError")
	generate!("Luau::AstStat")
	generate!("Luau::AstStatBlock")
	generate!("Luau::AstStatIf")
	generate!("Luau::AstStatWhile")
	generate!("Luau::AstStatRepeat")
	generate!("Luau::AstStatBreak")
	generate!("Luau::AstStatContinue")
	generate!("Luau::AstStatReturn")
	generate!("Luau::AstStatExpr")
	generate!("Luau::AstStatLocal")
	generate!("Luau::AstStatFor")
	generate!("Luau::AstStatForIn")
	generate!("Luau::AstStatAssign")
	generate!("Luau::AstStatCompoundAssign")
	generate!("Luau::AstStatFunction")
	generate!("Luau::AstStatLocalFunction")
	generate!("Luau::AstStatTypeAlias")
	generate!("Luau::AstStatDeclareFunction")
	generate!("Luau::AstStatDeclareGlobal")
	generate!("Luau::AstStatDeclareClass")
	generate!("Luau::AstStatError")
	generate!("Luau::AstType")

	// Had to remove some to avoid Rust's macro token invocation limit.
}

pub mod ast {
	pub use crate::ast_inner::*;
}

include_cpp! {
	#include "lua.h"

	name!(vm_inner)
	safety!(unsafe)

	generate!("lua_State")
	generate!("lua_Debug")
	generate!("lua_Callbacks")
}

pub mod vm {
	pub use crate::vm_inner::*;
}

pub use autocxx::{c_int, c_long, c_longlong, c_short, c_uchar, c_uint, c_ulong, c_ulonglong, c_ushort, c_void};