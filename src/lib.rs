extern crate libc;

pub mod ffi;
pub mod state;

#[cfg(test)]
mod tests {
    use super::state;
    use ffi::LuaOp;

    #[test]
    fn hello() {
        state::new()
            .pushinteger(123)
            .pushinteger(456)
            .arith(LuaOp::Add);
    }
}
