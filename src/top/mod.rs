pub enum LuaStatus {
	Ok = 0,
	Yield,
	Runtime, // ERRRUN
	Syntax,  // ERRSYNTAX
	Memory,  // ERRMEM
	ErrorHandling, // ERRERR
	Break // Yielded for a debug breakpoint
}

pub const REGISTRY_INDEX: i32 = -10000;
pub const ENVIRONMENT_INDEX: i32 = -10001;
pub const GLOBALS_INDEX: i32 = -10002;

pub const LUA_TNONE: i32 = -1;

pub enum LuaType {
	Nil = 0,
	Boolean,
	LightUserdata,
	Number,
	Vector,
	String,

	// Gc types
	Table,
	Function,
	Userdata,
	Thread,

	Proto,
	Upvalue,
	Deadkey
}