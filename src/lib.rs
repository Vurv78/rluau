use autocxx::include_cpp;

include_cpp! {
	#include "Luau/Compiler.h"

	name!(comp_inner)
	safety!(unsafe)

	generate_pod!("Luau::CompileOptions")
	generate_pod!("Luau::ParseOptions")

	generate!("Luau::CompileError")
	generate!("Luau::compileOrThrow")
	generate!("Luau::compile")
}

pub mod compiler {
	pub use crate::comp_inner::*;
}

impl Default for compiler::Luau::CompileOptions {
	fn default() -> Self {
		Self {
			bytecodeVersion: 1,
			optimizationLevel: 1,
			debugLevel: 1,
			coverageLevel: 0,
			vectorLib: std::ptr::null(),
			vectorCtor: std::ptr::null()
		}
	}
}

impl Default for compiler::Luau::ParseOptions {
	fn default() -> Self {
		Self {
			allowTypeAnnotations: true,
			supportContinueStatement: true,
			allowDeclarationSyntax: true,
			captureComments: true
		}
	}
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
	#include "lualib.h"

	name!(vm_inner)
	safety!(unsafe)

	// Enum
	generate_pod!("lua_Status")

	// Typedef
	generate!("lua_CFunction")
	generate!("lua_Alloc")

	// Struct
	generate!("lua_Debug")
	generate!("lua_Callbacks")

	// LUA_API functions
	generate!("lua_close")
	generate!("lua_newthread")
	generate!("lua_mainthread")
	generate!("lua_gettop")
	generate!("lua_settop")
	generate!("lua_pushvalue")
	generate!("lua_remove")
	generate!("lua_insert")
	generate!("lua_replace")
	generate!("lua_checkstack")
	generate!("lua_rawcheckstack")
	generate!("lua_xmove")
	generate!("lua_xpush")
	generate!("lua_isnumber")
	generate!("lua_isstring")
	generate!("lua_iscfunction")
	generate!("lua_isLfunction")
	generate!("lua_isuserdata")
	generate!("lua_type")
	generate!("lua_typename")
	generate!("lua_equal")
	generate!("lua_rawequal")
	generate!("lua_lessthan")
	generate!("lua_tonumberx")
	generate!("lua_tointegerx")
	generate!("lua_tounsignedx")
	generate!("lua_tovector")
	generate!("lua_toboolean")
	generate!("lua_tolstring")
	generate!("lua_tostringatom")
	generate!("lua_namecallatom")
	generate!("lua_objlen")
	generate!("lua_tocfunction")
	generate!("lua_touserdata")
	generate!("lua_touserdatatagged")
	generate!("lua_userdatatag")
	generate!("lua_tothread")
	generate!("lua_topointer")
	generate!("lua_pushnil")
	generate!("lua_pushnumber")
	generate!("lua_pushinteger")
	generate!("lua_pushunsigned")
	generate!("lua_pushvector")
	generate!("lua_pushlstring")
	generate!("lua_pushstring")
	generate!("lua_pushvfstring")
	generate!("lua_pushcfunction")
	generate!("lua_pushboolean")
	generate!("lua_pushlightuserdata")
	generate!("lua_pushthread")
	generate!("lua_gettable")
	generate!("lua_getfield")
	generate!("lua_rawgetfield")
	generate!("lua_rawget")
	generate!("lua_rawgeti")
	generate!("lua_createtable")
	generate!("lua_setreadonly")
	generate!("lua_getreadonly")
	generate!("lua_setsafeenv")
	generate!("lua_newuserdata")
	generate!("lua_newuserdatadtor")
	generate!("lua_getmetatable")
	generate!("lua_getfenv")
	generate!("lua_settable")
	generate!("lua_setfield")
	generate!("lua_rawset")
	generate!("lua_rawseti")
	generate!("lua_setmetatable")
	generate!("lua_setfenv")
	generate!("luau_load")
	generate!("lua_call")
	generate!("lua_pcall")
	generate!("lua_yield")
	generate!("lua_break")
	generate!("lua_resume")
	generate!("lua_resumeerror")
	generate!("lua_status")
	generate!("lua_isyieldable")
	generate!("lua_gc")
	generate!("lua_error")
	generate!("lua_next")
	generate!("lua_concat")
	generate!("lua_encodepointer")
	generate!("lua_clock")
	generate!("lua_setuserdatadtor")
	generate!("lua_ref")
	generate!("lua_unref")
	generate!("lua_getinfo")
	generate!("lua_getargument")
	generate!("lua_getlocal")
	generate!("lua_setlocal")
	generate!("lua_getupvalue")
	generate!("lua_setupvalue")
	generate!("lua_singlestep")
	generate!("lua_breakpoint")
	generate!("lua_debugtrace")
	generate!("lua_callbacks")
	generate!("lua_newstate")

	generate!("luaL_register")
	generate!("luaL_getmetafield")
	generate!("luaL_callmeta")
	generate!("luaL_typeerrorL")
	generate!("luaL_argerrorL")
	generate!("luaL_checklstring")
	generate!("luaL_optlstring")
	generate!("luaL_checknumber")
	generate!("luaL_optnumber")
	generate!("luaL_checkinteger")
	generate!("luaL_optinteger")
	generate!("luaL_checkunsigned")
	generate!("luaL_optunsigned")
	generate!("luaL_checkstack")
	generate!("luaL_checktype")
	generate!("luaL_checkany")
	generate!("luaL_newmetatable")
	generate!("luaL_checkudata")
	generate!("luaL_where")
	generate!("luaL_checkoption")
	generate!("luaL_tolstring")
	generate!("luaL_newstate")
	generate!("luaL_findtable")
	generate!("luaL_buffinit")
	generate!("luaL_buffinitsize")
	generate!("luaL_extendbuffer")
	generate!("luaL_reservebuffer")
	generate!("luaL_addlstring")
	generate!("luaL_addvalue")
	generate!("luaL_pushresult")
	generate!("luaL_pushresultsize")
	generate!("luaopen_base")
	generate!("luaopen_coroutine")
	generate!("luaopen_table")
	generate!("luaopen_os")
	generate!("luaopen_string")
	generate!("luaopen_bit32")
	generate!("luaopen_utf8")
	generate!("luaopen_math")
	generate!("luaopen_debug")
	generate!("luaL_openlibs")
	generate!("luaL_sandbox")
	generate!("luaL_sandboxthread")
}

pub mod vm {
	pub use crate::vm_inner::*;
}

pub use autocxx::{c_int, c_long, c_longlong, c_short, c_uchar, c_uint, c_ulong, c_ulonglong, c_ushort, c_void};