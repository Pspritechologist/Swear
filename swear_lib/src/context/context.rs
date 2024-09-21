use crate::runtime::operations::Operations;

use super::*;
use enum_dispatch::enum_dispatch;

// #[derive(Debug)]
#[enum_dispatch(Context)]
pub enum ContextHolder {
	ContextLevel,
	ObjectRef,
}

#[enum_dispatch]
pub trait Context {
	fn get(&self, key: &str) -> Option<ContextItem>;
	fn set(&mut self, key: String, value: ContextItem);
}

#[derive(Debug, Default)]
pub struct ContextLevel {
	pub items: std::collections::HashMap<String, ContextItem>,
	pub instructions: Expression,
	pub ops: Vec<Operations>,
}

impl ContextLevel {
	pub fn new(mut instructions: Expression) -> Self {
		instructions.reverse();
		Self {
			items: std::collections::HashMap::new(),
			instructions,
			ops: Vec::new(),
		}
	}
}

impl Context for ContextLevel {
	fn get(&self, key: &str) -> Option<ContextItem> {
		self.items.get(key).cloned()
	}

	fn set(&mut self, key: String, value: ContextItem) {
		self.items.insert(key, value);
	}
}
