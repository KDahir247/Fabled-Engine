#![feature(fn_traits)]
#![feature(unboxed_closures)]
#![feature(generic_const_exprs)]

extern crate core;

mod container;
mod error;
mod initialization;

pub use container::*;
pub use error::*;
pub use initialization::*;


#[cfg(test)]
mod tests {
    use mlua::{Lua, LuaOptions};

    #[test]
    fn it_works() {
        let lua = Lua::new();

        println!("{:?}", lua.load("1 + 1").eval::<i32>().unwrap());
        assert_eq!(lua.load("1 + 1").eval::<i32>().unwrap(), 2);
    }
}
