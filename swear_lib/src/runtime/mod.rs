pub mod operations;



pub use crate::context::ObjectRef;
use crate::object::*;
use crate::context::*;
use operations::Operations;
use swear_parser::{Definition, TopLevelItem, Valuable, Repetition, Expression};

pub trait SwearRuntime<'rt> {
	fn new(script: &'rt Expression) -> Self;
	fn step(&mut self);
	fn next_operation(&self) -> Option<&Operations<'rt>>;
	fn last_operation(&self) -> Option<&Operations<'rt>>;
	fn instruction_index(&self) -> usize;
	fn current_instruction(&self) -> Option<&TopLevelItem>;
	fn table(&self) -> &[ObjectRef<'rt>];
	fn stack(&self) -> &[ContextHolder<'rt>];
	fn is_finished(&self) -> bool;
	fn get_result(self) -> Option<Object<'rt>>;
}

#[derive(Debug)]
pub struct ContextStack<'rt> {
	stack: Vec<ContextHolder<'rt>>,
	at_root: bool,
	last_op: Option<Operations<'rt>>,
	table: Vec<ObjectRef<'rt>>,
	finished: bool,
	result: Option<Object<'rt>>,
}



impl<'rt> SwearRuntime<'rt> for ContextStack<'rt> {
	fn new(script: &'rt Expression) -> Self {
		Self {
			stack: vec![
				ContextLevel::<'rt>::new(&script).into()
			],
			at_root: true,
			last_op: None,
			table: Vec::new(),
			finished: false,
			result: None,
		}
	}

	fn step(&mut self) {
		if self.is_finished() {
			return;
		}

		while self.ops().is_empty() {

			let cont = self.runtime_cont();
			if cont.instr_index() >= cont.instructions().len() {
				self.pop();
				if self.is_finished() {
					return;
				}

				continue;
			}

			let instr = &cont.instructions()[cont.instr_index()];
			self.process_instructions(&instr);

			*self.runtime_cont_mut().instr_index_mut() += 1;
		}

		self.handle_next_op();
	}

	fn next_operation(&self) -> Option<&Operations<'rt>> {
		self.ops().last()
	}

	fn last_operation(&self) -> Option<&Operations<'rt>> {
		self.last_op.as_ref()
	}

	fn instruction_index(&self) -> usize {
		let ContextHolder::RuntimeContext(cont) = self.stack.first().unwrap() else {
			unreachable!("Stack had no root.");
		};

		cont.instr_index()
	}

	fn current_instruction(&self) -> Option<&TopLevelItem> {
		let ContextHolder::RuntimeContext(cont) = self.stack.first().unwrap() else {
			unreachable!("Stack had no root.");
		};

		cont.instructions().get(cont.instr_index())
	}

	fn table(&self) -> &[ObjectRef<'rt>] {
		&self.table
	}

	fn stack(&self) -> &[ContextHolder<'rt>] {
		&self.stack
	}

	fn is_finished(&self) -> bool {
		self.finished
	}

	fn get_result(self) -> Option<Object<'rt>> {
		self.result
	}
}

impl<'rt> ContextStack<'rt> {
	fn process_instructions(&mut self, instruction: &'rt TopLevelItem) {
		match instruction {
			TopLevelItem::Definition(d) => self.process_instr_definition(d),
			TopLevelItem::Valuable(v) => self.process_instr_valuable(v),
			TopLevelItem::Repetition(r) => self.process_instr_repetition(r),
			TopLevelItem::Dropper(value) => self.process_instr_dropper(value),
		}
	}

	fn process_instr_definition(&mut self, definition: &'rt Definition) {
		match definition {
			Definition::Blueprint { name, exprs } => {
				self.ops_mut().push(Operations::RegisterBlueprint {
					ident: name,
					expr: exprs,
				});
			},
			Definition::Callback { name, parameters, exprs } => {
				self.ops_mut().push(Operations::RegisterCallback {
					ident: name,
					parameters,
					expr: exprs,
				});
			}
			Definition::Register { name, value } => {
				self.ops_mut().push(Operations::RegisterObject(name));
				self.process_instr_valuable(value);
			},
		}
	}

	fn process_instr_valuable(&mut self, value: &'rt Valuable) {
		match value {
			Valuable::Expression(expr) => {
				// self.ops_mut().push(Operations::PopContext);
				self.ops_mut().push(Operations::PushContext(expr));
			},
			Valuable::ObjectLiteral(literal) => {
				self.ops_mut().push(Operations::PushObject(literal));
			},
			Valuable::ObjectConversion(obj_conv) => {
				self.ops_mut().push(Operations::ConvertObject(&obj_conv.symbol));
				self.process_instr_valuable(&obj_conv.value);
			},
			Valuable::Identifier(ident) => {
				self.ops_mut().push(Operations::PushIdentifier(ident));
			},
			Valuable::Callback(callback) => {
				self.ops_mut().push(Operations::ExCallback {
					method: callback.target.is_some(),
					callback: &callback.id,
					parameters: callback.parameters.len(),
				});
			
				if let Some(target) = callback.target.as_ref() {
					self.process_instr_valuable(&target);
				}

				for param in callback.parameters.iter() {
					self.process_instr_valuable(param);
				}
			},
		}
	}

	fn process_instr_dropper(&mut self, value: &'rt Option<Valuable>) {
		self.ops_mut().push(Operations::PopContext);

		if let Some(value) = value {
			self.process_instr_valuable(value);
		} else {
			self.ops_mut().push(Operations::PushObject(&swear_parser::ObjectLiteral::Zip));
		}
	}

	fn process_instr_repetition(&mut self, rep: &'rt Repetition) {
		self.ops_mut().push(Operations::Repeat(&rep.value));
		self.process_instr_valuable(&rep.cond);
	}

	fn push(&mut self, context: ContextHolder<'rt>) {
		self.stack.push(context);
		self.at_root = false;
	}

	fn pop(&mut self) {
		let result = self.table_mut().pop().unwrap_or_default();

		if self.at_root {
			self.finished = true;
			self.stack.clear();
			self.result = Some(std::sync::Arc::into_inner(result.inner()).unwrap().into_inner().unwrap());
			return;
		}

		if let ContextHolder::RuntimeContext(RuntimeContext::Blueprint(blueprint_cont)) = self.stack.pop().unwrap() {
			let mut obj = Dynamic::default();
			for (key, value) in blueprint_cont.items {
				obj.set(key, value);
			}

			self.table_mut().push(ObjectRef::new(obj.into()));
		} else {
			self.table_mut().push(result);
		};

		if self.stack.len() == 1 {
			self.at_root = true;
		}
	}

	// fn cont(&self) -> &ContextHolder {
	// 	self.stack.last().unwrap()
	// }

	fn cont_mut(&mut self) -> &mut ContextHolder<'rt> {
		self.stack.last_mut().unwrap()
	}

	fn runtime_cont(&self) -> &RuntimeContext<'rt> {
		let mut iter = self.stack.iter().rev();
		loop {
			if let ContextHolder::RuntimeContext(level) = iter.next().unwrap() {
				return level;
			}
		}
	}

	fn runtime_cont_mut(&mut self) -> &mut RuntimeContext<'rt> {
		let mut iter = self.stack.iter_mut().rev();
		loop {
			if let ContextHolder::RuntimeContext(level) = iter.next().unwrap() {
				return level;
			}
		}
	}

	fn ops(&self) -> &Vec<Operations<'rt>> {
		&self.runtime_cont().ops()
	}

	fn ops_mut(&mut self) -> &mut Vec<Operations<'rt>> {
		self.runtime_cont_mut().ops_mut()
	}

	// fn table(&self) -> &Vec<ObjectRef> {
	// 	&self.table
	// }

	fn table_mut(&mut self) -> &mut Vec<ObjectRef<'rt>> {
		&mut self.table
	}

	fn table_pop(&mut self) -> ObjectRef<'rt> {
		match self.table_mut().pop() {
			Some(obj) => obj,
			None => {
				panic!("Table is empty");
			}
		}
	}
}

impl<'rt> IContext<'rt> for ContextStack<'rt> {
	fn get(&self,key: &str) -> Option<ContextItem<'rt>> {
		for context in self.stack.iter().rev() {
			if let Some(value) = context.get(key) {
				return Some(value);
			}
		}

		None
	}

	fn set(&mut self, key: String, value: ContextItem<'rt>) {
		self.cont_mut().set(key, value);
	}
}
