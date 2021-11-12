// Raw bindings with no abstractions

use autocxx::{include_cpp, extern_rust::extern_rust_type};
use std::os::raw::{c_char, c_int, c_void, c_long, c_ushort};

include_cpp! {
	#include "Luau/Compiler.h"
	#include "Luau/Bytecode.h"

	name!(i_compiler)
	safety!(unsafe)

	generate_pod!("Luau::CompileOptions")
	generate_pod!("Luau::ParseOptions")

	generate!("Luau::CompileError")
	generate!("Luau::compileOrThrow")
	generate!("Luau::compile")

	generate!("LuauOpcode")
	generate!("LuauBytecodeTag")
	generate!("LuauBuiltinFunction")
	generate!("LuauCaptureType")
}

include_cpp! {
	#include "Luau/Ast.h"

	name!(i_ast)
	safety!(unsafe)

	generate!("Luau::AstVisitor")
	generate!("Luau::AstNode")
	generate!("Luau::AstExpr")
	generate!("Luau::AstStat")
	generate!("Luau::AstType")
}

// Struct that adds common fields shared by most types here
// First argument is the name of the struct
macro_rules! CommonStruct {
	($(#[$a:expr])* $name:ident $($field:ident: $ty:ty),*) => {
		struct $name {
			next: *mut GCObject,
			tt: u8,
			marked: u8,
			memcat: u8,
			$($field: $ty,)*
		}
	}
}

union TableUnion {
	lastfree: c_int,  /* any free position is before this position */
	aboundary: c_int /* negated 'boundary' of `array' array; iff aboundary < 0 */
}

struct LuaNode {
	val: TValue,
	key: TKey
}

CommonStruct! {
	Table

	flags: u8,      /* 1<<p means tagmethod(p) is not present */
	readonly: u8,   /* sandboxing feature to prohibit writes to table */
	safeenv: u8,    /* environment doesn't share globals with other scripts */
	lsizenode: u8,  /* log2 of size of `node' array */
	nodemask8: u8, /* (1<<lsizenode)-1, truncated to 8 bits */

	sizearray: c_int, /* size of `array' array */
	inner: TableUnion,


	metatable: *mut Table,
	array: *mut TValue,  /* array part */
	node: *mut LuaNode,
	gclist: *mut GCObject
}

struct TValue {
	value: Value,
	extra: c_int,
	tt: c_int
}

CommonStruct! {
	#[derive(Clone, Copy)] // beautiful
	GCHeader
}

#[derive(Clone, Copy)]
union LuaUserAlignmentT {
	u: f64, // double
	s: *mut c_void,
	l: c_long
}
union UDataUnion {
	data: [char; 1], // userdata is allocated right after the header
	dummy: LuaUserAlignmentT // ensures maximum alignment for data
}
CommonStruct! {
	UData
	//CommonHeader

	tag: u8,
	len: c_int,

	metatable: *mut Table,

	inner: UDataUnion
}

#[repr(C)]
union GCObject {
	gch: GCHeader,
	ts: TString,
	u: UData,
	cl: Closure,
	h: Table,
	p: Proto,
	uv: UpVal,
	th: LuaState // thread
}

struct StringTable {
	hash: *mut *mut GCObject,
	nuse: u32, // Element #
	size: i32
}

struct GlobalState {
	strt: StringTable,
}

CommonStruct! {
	#[derive(Clone, Copy)]
	TString

	atom: u16,
	hash: u32, // c_uint
	len: u32, // c_uint

	data: [i8; 1] // char[1]
}

CommonStruct! {
	#[extern_rust_type]
	LuaState

	status: u8,

	activememcat: u8,
	stackstate: u8,

	singlestep: bool,
	top: *mut TValue, // StkId
	base: *mut TValue, // StkId
	global: *mut GlobalState,
	ci: *mut CallInfo,
	stack_last: *mut TValue,
	stack: *mut TValue,
	end_ci: *mut CallInfo,
	base_ci: *mut CallInfo,

	stacksize: c_int,
	size_ci: c_int,
	nCcalls: c_ushort,
	baseCcalls: c_ushort,
	cachedslot: c_int,
	l_gt: TValue,
	env: TValue,
	openupval: *mut GCObject,
	gclist: *mut GCObject,
	namecall: *mut TString,
	userdata: *mut std::ffi::c_void
}

#[extern_rust_type]
type lua_CFunction = extern "C" fn (L: *mut LuaState) -> i32;

include_cpp! {
	#include "lua.h"
	#include "lualib.h"

	name!(i_vm)
	safety!(unsafe)

	// Enum
	generate_pod!("lua_Status")

	//generate!("lua_CFunction")
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

	//generate!("luaL_checkoption")
	//generate!("luau_load")
}

include_cpp! {
	#include "Luau/StringUtils.h"
	name!(i_utils)
	safety!(unsafe)

	generate!("Luau::format")
	generate!("Luau::vformat")
	generate!("Luau::hashRange")

	/*
	generate!("Luau::join")
	generate!("Luau::split")
	generate!("Luau::editDistance")
	generate!("Luau::startsWith")
	generate!("Luau::equalsLower")
	*/
}

include_cpp! {
	#include "Luau/Transpiler.h"
	#include "Luau/AstQuery.h"

	name!(i_analysis)
	safety!(unsafe)

	generate!("Luau::TranspileResult")
	generate!("Luau::transpileWithTypes")
	generate!("Luau::transpile")
	generate!("Luau::dump")

	generate!("Luau::ExprOrLocal")
	generate!("Luau::findNodeAtPosition")
	generate!("Luau::findExprAtPosition")
	generate!("Luau::findExprOrLocalAtPosition")

	/*
	generate!("Luau::findAstAncestryOfPosition")
	generate!("Luau::findScopeAtPosition")
	*/

	// TODO: Support TranspileResult transpile(std::string_view source, ParseOptions options = ParseOptions{});
}


pub mod vm;
pub mod compiler;
pub mod ast;
pub mod utils;
pub mod analysis;