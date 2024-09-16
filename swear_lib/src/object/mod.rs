mod chars;
mod count;
mod state;
mod zip;
mod deck;
mod map;
mod dynamic;

pub use chars::*;
pub use count::*;
pub use state::*;
pub use zip::*;
pub use deck::*;
pub use map::*;
pub use dynamic::*;

use enum_dispatch::enum_dispatch;
use swear_parser::ObjectLiteral;

use crate::context::ObjectRef;

#[enum_dispatch]
#[derive(Clone, PartialEq, Eq)]
pub enum Object {
	Chars,
	Count,
	State,
	Zip,
	Deck,
	Map,
	Dynamic,
}

impl Object {
	pub fn from_literal(literal: ObjectLiteral) -> Self {
		match literal {
			ObjectLiteral::Chars(c) => Chars::from(c.clone()).into(),
			ObjectLiteral::Count(c) => Count::from(c as i64).into(), //FIXME
			ObjectLiteral::State(s) => State::from(s).into(),
			ObjectLiteral::Zip => Zip.into(),
			ObjectLiteral::Deck(d) => Deck::from_iter_lit(d).into(),
			ObjectLiteral::Map(m) => Map::from_iter_lit(m).into(),
		}
	}
}

impl std::fmt::Debug for Object {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Object::Chars(c) => write!(f, "Obj({:?})", c),
			Object::Count(c) => write!(f, "Obj({:?})", c),
			Object::State(s) => write!(f, "Obj({:?})", s),
			Object::Zip(z) => write!(f, "Obj({:?})", z),
			Object::Deck(d) => write!(f, "Obj({:?})", d),
			Object::Map(m) => write!(f, "Obj({:?})", m),
			Object::Dynamic(d) => write!(f, "Obj({:?})", d),
		}
	}
}

impl Default for Object {
	fn default() -> Self {
		Zip.into()
	}
}

impl From<ObjectLiteral> for Object {
	fn from(value: ObjectLiteral) -> Self {
		Object::from_literal(value)
	}
}

#[enum_dispatch(Object)]
pub trait IObject {
	fn to_chars(&self) -> Chars;
	fn to_count(&self) -> Count;
	fn to_state(&self) -> State;
	fn to_zip(&self) -> Zip {
		Zip
	}
	fn to_deck(&self) -> Deck;
	fn to_map(&self) -> Map;

	fn is_zip(&self) -> bool {
		false
	}
	fn is_dynamic(&self) -> bool {
		false
	}

	fn object_name(&self) -> &str;
}

#[non_exhaustive]
pub struct FunctionInfo {
	pub name: String,
	pub arg_count: usize,
	pub function: Box<dyn FnMut(ObjectRef, Vec<ObjectRef>) -> Option<ObjectRef>>,
}

pub struct FunctionInfoBuilder {
	name: String,
	arg_count: usize,
}

impl FunctionInfoBuilder {
	pub fn new(name: String) -> Self {
		Self {
			name,
			arg_count: 0,
		}
	}

	pub fn with_arg_count(mut self, arg_count: usize) -> Self {
		self.arg_count = arg_count;
		self
	}

	pub fn build(self, function: Box<dyn FnMut(ObjectRef, Vec<ObjectRef>) -> Option<ObjectRef>>) -> FunctionInfo {
		FunctionInfo {
			name: self.name,
			arg_count: self.arg_count,
			function,
		}
	}
}
