extern crate lua;
extern crate std;

use lua::ffi::lua_State;
use libc::c_int;
use std::ptr::write;

use super::types::*;
use super::luatype::*;
use super::multireturn::MultiReturn::*;

pub unsafe extern "C" fn generic_gc<T>(ffi_state : *mut lua_State) -> c_int where T : LuaType {
	let mut state = lua::State::from_ptr(ffi_state);
	if let Some(ptr) = state.test_userdata_typed(1, T::type_id()) {
		let raw_ptr = ptr as *mut Option<LuaCell<T>>;
		*raw_ptr = None;
	}

	0
}

pub unsafe extern "C" fn generic_new<T>(ffi_state : *mut lua_State) -> c_int where T : LuaType {
	let mut state = lua::State::from_ptr(ffi_state);
	let v: *mut Option<LuaCell<T>> = state.new_userdata_typed();
	let obj = T::lua_new(&mut state);
	write(v, Some(std::rc::Rc::new(std::cell::RefCell::new(obj))));
	state.set_metatable_from_registry(T::type_id());
	1
}

pub unsafe fn unpack_item<T>(state : &mut lua::State) -> Option<LuaCell<T>> where T : LuaType {
	let foo : Option<&mut Option<LuaCell<T>>> = { state.test_userdata_typed(1, T::type_id()) };
	if let Some(&mut Some(ref bx)) = foo {
		Some(bx.clone())
	} else {
		None
	}
}

pub unsafe extern "C" fn generic_index<T>(ffi_state : *mut lua_State) -> c_int where T : LuaType {
	let mut state = lua::State::from_ptr(ffi_state);
	state.push_value(1);
	state.get_metatable_from_registry(T::type_id());
	// attempt to get from methods table
	state.push_string("methods");
	state.raw_get(-2);
	state.push_value(2);
	state.raw_get(-2);
	if state.is_fn(-1) {
		return 1;
	} else {
		state.pop(2);
	}

	// attempt to get from getters table
	state.push_string("getters");
	state.raw_get(-2);
	state.push_value(2);
	state.raw_get(-2);
	if state.is_fn(-1) {
		state.push_value(1);
		let size_before_call = state.get_top();
		state.call(1, lua::MULTRET);
		let size_after_call = state.get_top();
		return 2 + size_after_call - size_before_call;
	}

	return Err(String::from("no such index")).to_lua(&mut state);
}

pub unsafe extern "C" fn generic_newindex<T>(ffi_state : *mut lua_State) -> c_int where T : LuaType {
	let mut state = lua::State::from_ptr(ffi_state);
	state.push_value(1);
	state.get_metatable_from_registry(T::type_id());
	// attempt to get from setters table
	state.push_string("setters");
	state.raw_get(-2);
	state.push_value(2);
	state.raw_get(-2);
	if state.is_fn(-1) {
		state.push_value(1);
		state.push_value(3);
		let size_before_call = state.get_top();
		state.call(2, lua::MULTRET);
		let size_after_call = state.get_top();
		return 2 + size_after_call - size_before_call;
	}

	return Err(String::from("no such setter")).to_lua(&mut state);
}
