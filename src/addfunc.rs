extern crate lua;

use super::types::BareLuaFunction;

pub trait AddFunc {
	fn add_func(&mut self, key : &str, func : BareLuaFunction);
}

impl AddFunc for lua::State {
	fn add_func(&mut self, key : &str, func : BareLuaFunction) {
		self.push_string(key);
		self.push_closure(Some(func), 0);
		self.raw_set(-3);
	}
}
