use luau_src::compiler as raw;

#[derive(Default)]
#[repr(transparent)]
pub struct CompileOptions(pub raw::lua_CompileOptions);
