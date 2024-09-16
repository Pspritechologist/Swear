use crate::runtime::operations::Operations;

use super::*;
use swear_parser::TopLevelItem;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ContextItem {
	Object(ObjectRef),
	Callback(),
	Blueprint(),
}

pub trait Context {
	fn get(&self, key: &str) -> Option<ObjectRef>;
	fn set(&mut self, key: String, value: ObjectRef);
}

#[derive(Debug, Default)]
pub struct ContextLevel {
	pub items: std::collections::HashMap<String, ContextItem>,
	pub instructions: Vec<TopLevelItem>,
	pub instr_index: usize,
	pub ops: Vec<Operations>,
	pub table: Vec<ObjectRef>,
}

impl ContextLevel {
	pub fn new(instructions: Vec<TopLevelItem>) -> Self {
		Self {
			items: std::collections::HashMap::new(),
			instructions,
			instr_index: 0,
			ops: Vec::new(),
			table: Vec::new(),
		}
	}
}

impl Context for ContextLevel {
	fn get(&self, key: &str) -> Option<ObjectRef> {
		match self.items.get(key) {
			Some(ContextItem::Object(object)) => Some(object.clone()),
			Some(ContextItem::Callback()) => unimplemented!("Callback"),
			Some(ContextItem::Blueprint()) => unimplemented!("Blueprint"),
			None => None,
		}
	}

	fn set(&mut self, key: String, value: ObjectRef) {
		self.items.insert(key, ContextItem::Object(value));
	}
}
