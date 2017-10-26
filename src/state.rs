use std::marker::PhantomData;
use std::mem;
use std::ops::Drop;
use ffi;

pub struct Cons<A, B>(PhantomData<(A, B)>);
pub struct Nil;

pub struct Integer;

pub trait Head {
    type Ty;
}

impl<T, B> Head for Cons<T, B> {
    type Ty = T;
}

pub trait Tail: Sized {
    type Ty;
}

impl<H, T> Tail for Cons<H, T> {
    type Ty = T;
}

pub struct State<T>(*mut ffi::LuaState, PhantomData<T>);

pub fn new() -> State<Nil> {
    unsafe {
        let state = ffi::luaL_newstate();
        ffi::luaL_openlibs(state);
        State(state, PhantomData)
    }
}

impl<T> State<T> {
    fn return_state<U>(self) -> State<U> {
        let state = State(self.0, PhantomData);
        mem::forget(self);
        state
    }

    pub fn run<F: FnOnce(Self) -> Self>(self, f: F) -> Self {
        f(self)
    }

    pub fn pushinteger(self, int: ffi::LuaInteger) -> State<Cons<Integer, T>>
    {
        unsafe { ffi::lua_pushinteger(self.0, int) };
        self.return_state()
    }

    pub fn arith<Ret>(self, op: ffi::LuaOp) -> State<Ret>
        where T : Tail<Ty = Ret>
    {
        unsafe { ffi::lua_arith(self.0, op) };
        self.return_state()
    }
}

impl<T> Drop for State<T> {
    fn drop(&mut self) {
        unsafe { ffi::lua_close(self.0) }
    }
}
