mod chars;
mod count;
mod state;
mod zip;
mod deck;
mod map;
mod dynamic;

use std::sync::{Arc, Mutex};
use rustc_hash::FxHashMap as HashMap;

pub use chars::*;
pub use count::*;
pub use state::*;
pub use zip::*;
pub use deck::*;
pub use map::*;
pub use dynamic::*;

pub type ObjectFunction<'rt> = dyn FnMut(ObjectRef<'rt>, Vec<ObjectRef<'rt>>) -> Result<Option<ObjectRef<'rt>>, ()>;

use enum_dispatch::enum_dispatch;
use swear_parser::ObjectLiteral;
use swear_lib_macros::swear_object;

use crate::context::{Callback, NativeCallback, ObjectRef};

#[enum_dispatch]
#[derive(Clone)]
#[derive(enum_as_inner::EnumAsInner)]
pub enum Object<'rt> {
	Chars,
	Count,
	State,
	Zip,
	Deck(Deck<'rt>),
	Map(Map<'rt>),
	Dynamic(Dynamic<'rt>),
}

impl<'rt> PartialEq for Object<'rt> {
	fn eq(&self, other: &Self) -> bool {
		match (self, other) {
			(Object::Chars(a), Object::Chars(b)) => a == b,
			(Object::Count(a), Object::Count(b)) => a == b,
			(Object::State(a), Object::State(b)) => a == b,
			(Object::Zip(a), Object::Zip(b)) => a == b,
			(Object::Deck(a), Object::Deck(b)) => a == b,
			(Object::Map(a), Object::Map(b)) => a == b,
			(Object::Dynamic(_), Object::Dynamic(_)) => false,
			_ => false,
		}
	}
}
impl<'rt> Eq for Object<'rt> {}

impl<'rt> Object<'rt> {
	pub fn from_literal(literal: &ObjectLiteral) -> Self {
		match literal {
			ObjectLiteral::Chars(c) => Chars::from(c.clone()).into(),
			ObjectLiteral::Count(c) => Count::from(*c as i64).into(), //FIXME
			ObjectLiteral::State(s) => State::from(*s).into(),
			ObjectLiteral::Zip => Zip.into(),
			ObjectLiteral::Deck(d) => Deck::from_vec_lit(d).into(),
			ObjectLiteral::Map(m) => Map::from_vec_lit(m).into(),
		}
	}
}

impl<'rt> std::fmt::Debug for Object<'rt> {
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

impl<'rt> Default for Object<'rt> {
	fn default() -> Self {
		Zip.into()
	}
}

// impl From<ObjectLiteral> for Object {
// 	fn from(value: ObjectLiteral) -> Self {
// 		Object::from_literal(&value)
// 	}
// }

#[enum_dispatch(Object)]
pub trait IObject<'rt> {
	fn to_chars(&self) -> Chars;
	fn to_count(&self) -> Count;
	fn to_state(&self) -> State;
	fn to_zip(&self) -> Zip {
		Zip
	}
	fn to_deck(&self) -> Deck<'rt>;
	fn to_map(&self) -> Map<'rt>;

	fn is_zip(&self) -> bool {
		false
	}
	fn is_dynamic(&self) -> bool {
		false
	}

	fn get_info(&self) -> ObjectInfo;

	fn get_functions(&self) -> HashMap<String, FunctionInfo<'rt>> {
		HashMap::default()
	}
}

#[non_exhaustive]
#[derive(Clone, Debug)]
pub struct ObjectInfo {
	pub name: String,
	pub description: Option<String>,
}

impl ObjectInfo {
	pub fn from_str(name: &str) -> Self {
		Self {
			name: name.into(),
			description: None,
		}
	}

	pub fn from_string(name: String) -> Self {
		Self {
			name,
			description: None,
		}
	}

	pub fn with_description_str(mut self, description: &str) -> Self {
		self.description = Some(description.into());
		self
	}

	pub fn with_description_string(mut self, description: String) -> Self {
		self.description = Some(description);
		self
	}

	pub fn with_description(mut self, description: Option<String>) -> Self {
		self.description = description;
		self
	}
}

#[non_exhaustive]
#[derive(Clone, Debug)]
pub struct FunctionInfo<'rt> {
	pub name: String,
	pub function: Callback<'rt>,
}

pub struct FunctionInfoBuilder {
	name: String,
	arg_count: usize,
}

impl<'rt> FunctionInfoBuilder {
	pub fn new(name: String) -> Self {
		Self {
			name,
			arg_count: 0,
		}
	}

	pub fn build_native(self, function: Arc<Mutex<ObjectFunction<'rt>>>) -> FunctionInfo<'rt> {
		FunctionInfo {
			name: self.name,
			function: NativeCallback {
				arg_count: self.arg_count,
				callback: function,
			}.into(),
		}
	}

	pub fn build(self, callback: Callback<'rt>) -> FunctionInfo<'rt> {
		FunctionInfo {
			name: self.name,
			function: callback,
		}
	}
}

#[cfg(feature="serde")]
mod serde_impl {
	use super::*;

	impl<'rt> serde::Serialize for Object<'rt> {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: serde::Serializer,
		{
			match self {
				Object::Chars(c) => c.serialize(serializer),
				Object::Count(c) => c.serialize(serializer),
				Object::State(s) => s.serialize(serializer),
				Object::Zip(z) => z.serialize(serializer),
				Object::Deck(d) => d.serialize(serializer),
				Object::Map(m) => m.serialize(serializer),
				Object::Dynamic(d) => d.serialize(serializer),
			}
		}
	}

	impl<'de, 'rt> serde::Deserialize<'de> for Object<'rt> {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: serde::Deserializer<'de>,
		{
			let value = ObjectLiteral::deserialize(deserializer)?;
			Ok(Object::from_literal(&value))
		}
	}
}
