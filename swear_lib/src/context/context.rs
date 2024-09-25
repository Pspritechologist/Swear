use crate::runtime::operations::Operations;
use std::collections::BTreeMap as HashMap;

use super::*;
use enum_dispatch::enum_dispatch;

#[derive(Clone, Debug)]
#[enum_dispatch(IContext)]
pub enum ContextHolder<'rt> {
	RuntimeContext(RuntimeContext<'rt>),
	ObjectRef(ObjectRef<'rt>),
}

impl<'rt> From<ContextLevel<'rt>> for ContextHolder<'rt> {
	fn from(context: ContextLevel<'rt>) -> Self {
		ContextHolder::RuntimeContext(RuntimeContext::ContextLevel(context))
	}
}
impl<'rt> From<BlueprintContext<'rt>> for ContextHolder<'rt> {
	fn from(context: BlueprintContext<'rt>) -> Self {
		ContextHolder::RuntimeContext(RuntimeContext::Blueprint(context))
	}
}

#[derive(Clone, Debug)]
#[enum_dispatch(IContext)]
#[enum_dispatch(IRuntimeContext)]
pub enum RuntimeContext<'rt> {
	ContextLevel(ContextLevel<'rt>),
	Blueprint(BlueprintContext<'rt>),
}

impl<'rt> IntoIterator for RuntimeContext<'rt> {
	type Item = (String, ContextItem<'rt>);
	type IntoIter = std::collections::btree_map::IntoIter<String, ContextItem<'rt>>;
	fn into_iter(self) -> Self::IntoIter {
		match self {
			RuntimeContext::ContextLevel(context) => context.into_iter(),
			RuntimeContext::Blueprint(context) => context.into_iter(),
		}
	}
}

#[enum_dispatch]
pub trait IContext<'rt> {
	fn get(&self, key: &str) -> Option<ContextItem<'rt>>;
	fn set(&mut self, key: String, value: ContextItem<'rt>);
}

impl<'rt> IntoIterator for ContextHolder<'rt> {
	type Item = (String, ContextItem<'rt>);
	type IntoIter = std::collections::btree_map::IntoIter<String, ContextItem<'rt>>;
	fn into_iter(self) -> Self::IntoIter {
		match self {
			ContextHolder::RuntimeContext(context) => context.into_iter(),
			ContextHolder::ObjectRef(context) => context.into_iter(),
		}
	}
}

#[enum_dispatch]
pub trait IRuntimeContext<'rt> {
	fn ops(&self) -> &Vec<Operations<'rt>>;
	fn ops_mut(&mut self) -> &mut Vec<Operations<'rt>>;
	fn instructions(&self) -> &'rt Expression;
	fn instr_index(&self) -> usize;
	fn instr_index_mut(&mut self) -> &mut usize;
}

#[derive(Clone, Debug)]
pub struct ContextLevel<'rt> {
	pub items: HashMap<String, ContextItem<'rt>>,
	pub instructions: &'rt Expression,
	pub instr_index: usize,
	pub ops: Vec<Operations<'rt>>,
}

impl<'rt> IntoIterator for ContextLevel<'rt> {
	type Item = (String, ContextItem<'rt>);
	type IntoIter = std::collections::btree_map::IntoIter<String, ContextItem<'rt>>;
	fn into_iter(self) -> Self::IntoIter {
		self.items.into_iter()
	}
}

impl<'rt> ContextLevel<'rt> {
	pub fn new(instructions: &'rt Expression) -> Self {
		Self {
			items: HashMap::default(),
			instructions,
			instr_index: 0,
			ops: Vec::new(),
		}
	}
}

impl<'rt> IContext<'rt> for ContextLevel<'rt> {
	fn get(&self, key: &str) -> Option<ContextItem<'rt>> {
		self.items.get(key).cloned()
	}

	fn set(&mut self, key: String, value: ContextItem<'rt>) {
		self.items.insert(key, value);
	}
}

impl<'rt> IRuntimeContext<'rt> for ContextLevel<'rt> {
	fn ops(&self) -> &Vec<Operations<'rt>> {
		&self.ops
	}
	fn ops_mut(&mut self) -> &mut Vec<Operations<'rt>> {
		&mut self.ops
	}
	fn instructions(&self) -> &'rt Expression {
		self.instructions
	}
	fn instr_index(&self) -> usize {
		self.instr_index
	}
	fn instr_index_mut(&mut self) -> &mut usize {
		&mut self.instr_index
	}
}

#[derive(Clone, Debug)]
pub struct BlueprintContext<'rt> {
	pub items: HashMap<String, ContextItem<'rt>>,
	pub instructions: &'rt Expression,
	pub instr_index: usize,
	pub ops: Vec<Operations<'rt>>,
}

impl<'rt> IntoIterator for BlueprintContext<'rt> {
	type Item = (String, ContextItem<'rt>);
	type IntoIter = std::collections::btree_map::IntoIter<String, ContextItem<'rt>>;
	fn into_iter(self) -> Self::IntoIter {
		self.items.into_iter()
	}
}

impl<'rt> BlueprintContext<'rt> {
	pub fn new(instructions: &'rt Expression) -> Self {
		Self {
			items: HashMap::default(),
			instructions,
			instr_index: 0,
			ops: Vec::new(),
		}
	}
}

impl<'rt> IContext<'rt> for BlueprintContext<'rt> {
	fn get(&self, key: &str) -> Option<ContextItem<'rt>> {
		self.items.get(key).cloned()
	}

	fn set(&mut self, key: String, value: ContextItem<'rt>) {
		self.items.insert(key, value);
	}
}

impl<'rt> IRuntimeContext<'rt> for BlueprintContext<'rt> {
	fn ops(&self) -> &Vec<Operations<'rt>> {
		&self.ops
	}
	fn ops_mut(&mut self) -> &mut Vec<Operations<'rt>> {
		&mut self.ops
	}
	fn instructions(&self) -> &'rt Expression {
		self.instructions
	}
	fn instr_index(&self) -> usize {
		self.instr_index
	}
	fn instr_index_mut(&mut self) -> &mut usize {
		&mut self.instr_index
	}
}
