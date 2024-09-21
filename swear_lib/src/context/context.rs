use crate::runtime::operations::Operations;

use super::*;
use enum_dispatch::enum_dispatch;

#[derive(Debug)]
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

#[derive(Debug)]
#[enum_dispatch(IContext)]
#[enum_dispatch(IRuntimeContext)]
pub enum RuntimeContext<'rt> {
	ContextLevel(ContextLevel<'rt>),
	Blueprint(BlueprintContext<'rt>),
}

#[enum_dispatch]
pub trait IContext<'rt> {
	fn get(&self, key: &str) -> Option<ContextItem<'rt>>;
	fn set(&mut self, key: String, value: ContextItem<'rt>);
}

#[enum_dispatch]
pub trait IRuntimeContext<'rt> {
	fn ops(&self) -> &Vec<Operations<'rt>>;
	fn ops_mut(&mut self) -> &mut Vec<Operations<'rt>>;
	fn instructions(&self) -> &'rt Expression;
	fn instr_index(&self) -> usize;
	fn instr_index_mut(&mut self) -> &mut usize;
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

#[derive(Debug)]
pub struct BlueprintContext<'rt> {
	pub items: std::collections::HashMap<String, ContextItem<'rt>>,
	pub instructions: &'rt Expression,
	pub instr_index: usize,
	pub ops: Vec<Operations<'rt>>,
}

impl<'rt> BlueprintContext<'rt> {
	pub fn new(instructions: &'rt Expression) -> Self {
		Self {
			items: std::collections::HashMap::new(),
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
