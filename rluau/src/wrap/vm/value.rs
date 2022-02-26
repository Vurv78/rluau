use super::prelude::*;
use super::types;

#[derive(Debug)]
pub enum Value<'luau> {
	Nil,

	Boolean(bool),

	LightUserData(types::LightUserdata),

	/// Note there are no Integers in Luau as there are in Lua 5.3
	Number(LuauNumber),

	/// Note the lifetime of the string is not monitored or tied to the context, at least for now.
	String(types::String<'luau>),

	Function(types::Function<'luau>),

	// Really needs a reference system to exist. However, that also can't exist since luau doesn't have lua_getallocf
	Table( () ),
}