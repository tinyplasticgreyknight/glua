use lua::ffi::lua_State;
use libc::c_int;
use std::cell::RefCell;
use std::rc::Rc;

pub type ReturnCount = i32;
pub type LuaCell<T> = Rc<RefCell<T>>;
pub type BareLuaFunction = unsafe extern "C" fn(ffi_state : *mut lua_State) -> c_int;
