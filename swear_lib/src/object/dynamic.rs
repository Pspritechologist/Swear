use crate::{context::{self, ContextItem}, runtime::ObjectRef};

use super::*;

#[derive(Default)]
pub struct Dynamic {
	name: String,
	description: Option<String>,
	contents: std::collections::HashMap<String, ContextItem>,
	// functions: std::collections::HashMap<String, FunctionInfo>,
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

	fn get_info(&self) -> ObjectInfo {
		ObjectInfo::from_str(&self.name)
			.with_description(self.description.clone())
	}

	// fn get_functions(&self) -> HashMap<String, FunctionInfo> {
	// 	self.functions
	// }
}

impl context::Context for Dynamic {
	fn get(&self, key: &str) -> Option<ContextItem> {
		self.contents.get(key).cloned()
	}

	fn set(&mut self, key: String, value: ContextItem) {
		self.contents.insert(key, value);
	}
}

impl std::fmt::Debug for Dynamic {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "Dynamic({:?})", self.contents)
	}
}
