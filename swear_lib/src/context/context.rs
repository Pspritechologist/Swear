use crate::runtime::operations::Operations;

use super::*;
use enum_dispatch::enum_dispatch;

// #[derive(Debug)]
#[enum_dispatch(Context)]
pub enum ContextHolder<'rt> {
	ContextLevel(ContextLevel<'rt>),
	ObjectRef(ObjectRef<'rt>),
}

#[enum_dispatch]
pub trait Context<'rt> {
	fn get(&self, key: &str) -> Option<ContextItem<'rt>>;
	fn set(&mut self, key: String, value: ContextItem<'rt>);
}

#[derive(Debug)]
pub struct ContextLevel<'rt> {
	pub items: std::collections::HashMap<String, ContextItem<'rt>>,
	pub instructions: &'rt Expression,
	pub instr_index: usize,
	pub ops: Vec<Operations<'rt>>,
}

impl<'rt> ContextLevel<'rt> {
	pub fn new(instructions: &'rt Expression) -> Self {
		Self {
			items: std::collections::HashMap::new(),
			instructions,
			instr_index: 0,
			ops: Vec::new(),
		}
	}
}

impl<'rt> Context<'rt> for ContextLevel<'rt> {
	fn get(&self, key: &str) -> Option<ContextItem<'rt>> {
		self.items.get(key).cloned()
	}

	fn set(&mut self, key: String, value: ContextItem<'rt>) {
		self.items.insert(key, value);
	}
}
