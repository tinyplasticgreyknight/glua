extern crate lua;

use lua::ffi::lua_State;
use libc::c_int;
use std::cell::{RefCell, Ref, RefMut};

use super::types::*;
use super::multireturn::*;
use super::multireturn::MultiReturn::*;
use super::generic_metamethods::*;

pub trait LuaType : Sized {
	fn type_id() -> &'static str;
	fn lua_new(state : &mut lua::State) -> Self;
	fn methods(state : &mut lua::State);
	fn getters(state : &mut lua::State);
	fn setters(state : &mut lua::State);

	fn register_type(state : &mut lua::State) -> MultiReturn {
		Self::register_metatable(state);
		state.pop(1);
		MultiReturn::Value
	}

	fn register_metatable(state : &mut lua::State) {
		state.new_metatable(Self::type_id());
		state.push_closure(Some(generic_gc::<Self>), 0);
		state.set_field(-2, "__gc");
		state.new_table();
		Self::methods(state);
		state.set_field(-2, "methods");
		state.new_table();
		Self::getters(state);
		state.set_field(-2, "getters");
		state.new_table();
		Self::setters(state);
		state.set_field(-2, "setters");
		state.push_closure(Some(generic_index::<Self>), 0);
		state.set_field(-2, "__index");
		state.push_closure(Some(generic_newindex::<Self>), 0);
		state.set_field(-2, "__newindex");
        }

	unsafe fn inner_call_method<F>(state : &mut lua::State, method : F) -> MultiReturn where F : Fn(LuaCell<Self>, &mut lua::State) -> MultiReturn {
		if let Some(cell) = unpack_item::<Self>(state) {
			method(cell, state)
		} else {
			Err(String::from("bad self argument"))
		}
	}

	unsafe fn call_method<F>(ffi_state : *mut lua_State, method : F) -> c_int where F : Fn(LuaCell<Self>, &mut lua::State) -> MultiReturn {
		let mut state = lua::State::from_ptr(ffi_state);
		let result = Self::inner_call_method(&mut state, method);
		result.to_lua(&mut state)
	}

	fn borrow_mut(state : &mut lua::State, index : lua::Index) -> Option<RefMut<Self>> {
		unsafe {
			state.test_userdata_typed::<RefCell<Self>>(index, Self::type_id()).map(|x| x.borrow_mut())
		}
	}

	fn borrow(state : &mut lua::State, index : lua::Index) -> Option<Ref<Self>> {
		unsafe {
			state.test_userdata_typed::<RefCell<Self>>(index, Self::type_id()).map(|x| x.borrow())
		}
	}
}
