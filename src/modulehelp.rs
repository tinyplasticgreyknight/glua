extern crate lua;

use lua::ffi::lua_State;
use libc::c_int;

use super::types::BareLuaFunction;
use super::{LuaType, MultiReturn};
use super::generic_new;

pub enum ModuleContext {
	Okay(lua::State),
	Error(lua::State, String),
}

pub trait LuaModule {
	fn setup(context : ModuleContext) -> ModuleContext;
}

pub fn luaopen<T>(ffi_state : *mut lua_State) -> c_int where T : LuaModule {
	T::setup(ModuleContext::new(ffi_state)).finish()
}

impl ModuleContext {
	pub fn new(ffi_state : *mut lua_State) -> ModuleContext {
		let mut state = unsafe { lua::State::from_ptr(ffi_state) };
		state.new_table();
		ModuleContext::Okay(state)
	}

	fn and_then<F>(self, func : F) -> ModuleContext where F : FnOnce(lua::State) -> ModuleContext {
		match self {
			ModuleContext::Okay(state) => func(state),
			a => a
		}
	}

	pub fn add_submodule<T>(self, name : &'static str) -> ModuleContext where T : LuaModule {
		self.and_then(|mut state| {
			state.new_table();
			match T::setup(ModuleContext::Okay(state)) {
				ModuleContext::Okay(mut state) => {
					state.set_field(-2, name);
					ModuleContext::Okay(state)
				},
				a => a
			}
		})
	}

	pub fn add_func(self, name : &'static str, func : BareLuaFunction) -> ModuleContext {
		self.and_then(|mut state| {
			state.push_closure(Some(func), 0);
			state.set_field(-2, name);
			ModuleContext::Okay(state)
		})
	}

	pub fn add_type<T>(self) -> ModuleContext where T : LuaType {
		self.and_then(|mut state| {
			match T::register_type(&mut state) {
				MultiReturn::Err(msg) => ModuleContext::Error(state, msg),
				_ => ModuleContext::Okay(state)
			}
		})
	}

	pub fn add_type_with_new<T>(self) -> ModuleContext where T : LuaType {
		self.add_type_with_constructor::<T>("new")
	}

	pub fn add_type_with_constructor<T>(self, ctor_name : &'static str) -> ModuleContext where T : LuaType {
		self.add_type::<T>().add_func(ctor_name, generic_new::<T>)
	}

	pub fn finish(self) -> c_int {
		match self {
			ModuleContext::Okay(_) => 1,
			ModuleContext::Error(mut state, s) => MultiReturn::Err(s).to_lua(&mut state),
		}
	}
}
