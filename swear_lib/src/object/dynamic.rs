use crate::{context::{self, ContextItem}, runtime::ObjectRef};

use super::*;

#[derive(Clone, Default, PartialEq, Eq)]
pub struct Dynamic {
	def_name: String,
	contents: std::collections::HashMap<String, ContextItem>,
}

impl IObject for Dynamic {
	fn to_chars(&self) -> Chars {
		Chars::default()
	}

	fn to_count(&self) -> Count {
		Count::default()
	}

	fn to_state(&self) -> State {
		State::default()
	}

	fn to_deck(&self) -> Deck {
		Deck::default()
	}

	fn to_map(&self) -> Map {
		Map::default()
	}

	fn is_dynamic(&self) -> bool {
		true
	}

	fn object_name(&self) -> &str {
		self.def_name.as_str()
	}
}

impl context::Context for Dynamic {
	fn get(&self, key: &str) -> Option<ObjectRef> {
		match self.contents.get(key) {
			Some(ContextItem::Object(object)) => Some(object.clone()),
			Some(ContextItem::Callback()) => unimplemented!("Callback"),
			Some(ContextItem::Blueprint()) => unimplemented!("Blueprint"),
			None => None,
		}
	}

	fn set(&mut self, key: String, value: ObjectRef) {
		self.contents.insert(key, ContextItem::Object(value));
	}
}

impl std::fmt::Debug for Dynamic {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "Dynamic({:?})", self.contents)
	}
}
