pub mod operations;

pub use crate::context::ObjectRef;

use crate::object::*;
use crate::context::*;
use operations::Operations;
use swear_parser::Expression;
use swear_parser::Repetition;
use swear_parser::{Definition, TopLevelItem, Valuable};

// #[derive(Debug)]
pub struct ContextStack<'rt> {
	// script: Expression,
	stack: Vec<ContextHolder<'rt>>,
	at_root: bool,
	table: Vec<ObjectRef<'rt>>,
	finished: bool,
	result: Option<Object<'rt>>,
}

pub trait SwearRuntime<'rt> {
	fn new(script: &'rt Expression) -> Self;
	fn step(&mut self);
	fn is_finished(&self) -> bool;
	fn get_result(self) -> Option<Object<'rt>>;
}

impl<'rt> SwearRuntime<'rt> for ContextStack<'rt> {
	fn new(script: &'rt Expression) -> Self {
		let mut stack = Self {
			stack: Vec::new(),
			at_root: true,
			table: Vec::new(),
			finished: false,
			result: None,
		};

		stack.stack.push(ContextLevel::<'rt>::new(&script).into());
		stack
	}

	fn step(&mut self) {
		while self.ops().is_empty() {
			let cont = self.cont_level();
			if cont.instr_index >= cont.instructions.len() {
				let obj = self.table_mut().pop().unwrap_or_else(|| Object::default().into());
				self.pop(obj);
				continue;
			}

			let instr = &cont.instructions[cont.instr_index];
			self.process_instructions(&instr);

			self.cont_level_mut().instr_index += 1;
		}

		self.handle_next_op();
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
			Definition::Blueprint { name: _, exprs: _ } => {
				todo!();
				// self.ops_mut().push(Operations::RegisterBlueprint {
				// 	ident: name,
				// 	expr: exprs,
				// });
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

	fn pop(&mut self, result: ObjectRef<'rt>) {
		if self.at_root {
			self.finished = true;
			self.stack.clear();
			self.result = Some(std::sync::Arc::into_inner(result.inner()).unwrap().into_inner().unwrap());
			return;
		}

		self.stack.pop();

		self.table_mut().push(result);

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

	fn cont_level(&self) -> &ContextLevel<'rt> {
		let mut iter = self.stack.iter().rev();
		loop {
			if let ContextHolder::ContextLevel(level) = iter.next().unwrap() {
				return level;
			}
		}
	}

	fn cont_level_mut(&mut self) -> &mut ContextLevel<'rt> {
		let mut iter = self.stack.iter_mut().rev();
		loop {
			if let ContextHolder::ContextLevel(level) = iter.next().unwrap() {
				return level;
			}
		}
	}

	fn ops(&self) -> &Vec<Operations<'rt>> {
		&self.cont_level().ops
	}

	fn ops_mut(&mut self) -> &mut Vec<Operations<'rt>> {
		&mut self.cont_level_mut().ops
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

impl<'rt> Context<'rt> for ContextStack<'rt> {
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
