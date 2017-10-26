use libc::ptrdiff_t;

pub type LuaInteger = ptrdiff_t;

pub enum LuaState {}

#[repr(C)]
pub enum LuaOp {
    Add = 0,
    Sub = 1,
}

#[link(name="lua")]
extern {
    pub fn luaL_newstate() -> *mut LuaState;
    pub fn luaL_openlibs(l: *mut LuaState);
    pub fn lua_close(l: *mut LuaState);
    pub fn lua_pushinteger(l: *mut LuaState, int: LuaInteger);
    pub fn lua_arith(l: *mut LuaState, op: LuaOp);
}
