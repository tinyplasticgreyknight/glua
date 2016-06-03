extern crate lua;

use libc::c_int;
use self::MultiReturn::*;

use super::types::*;

pub enum MultiReturn {
	Nothing,
	Value,
	Values(ReturnCount),
	Err(String),
}

impl MultiReturn {
	pub fn to_lua(self, state : &mut lua::State) -> c_int {
		match self {
			Nothing => 0,
			Value => 1,
			Values(n) => n,
			Err(s) => {
				state.push_string(&s);
				state.error()
			}
		}
	}

	pub fn and_then<F>(self, func : F) -> Self where F : FnOnce() -> Self {
		match self {
			Err(s) => Err(s),
			_ => func(),
		}
	}
}
