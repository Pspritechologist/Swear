pub mod operations;

pub use crate::context::ObjectRef;

use crate::object::*;
use crate::context::*;
use operations::Operations;
use swear_parser::Expression;
use swear_parser::{Definition, TopLevelItem, Valuable};

// #[derive(Debug)]
pub struct ContextStack {
	stack: Vec<ContextHolder>,
	at_root: bool,
	table: Vec<ObjectRef>,
}

pub trait SwearRuntime {
	fn new(script: Expression) -> Self;
	fn step(&mut self) -> Option<Object>;
}

impl SwearRuntime for ContextStack {
	fn new(script: Expression) -> Self {
		let mut stack = Self {
			stack: Vec::new(),
			at_root: true,
			table: Vec::new(),
		};

		stack.stack.push(ContextLevel::new(script).into());
		stack
	}

	fn step(&mut self) -> Option<Object> {
		while self.ops().is_empty() {
			let cont = self.cont_level();
			if cont.instructions.is_empty() {
				if !self.at_root {
					let obj = self.table_mut().pop().unwrap_or_else(|| Object::default().into());
					self.pop();
					self.table_mut().push(obj);
					continue;
				} else {
					return None;
				}
			}

			let instr = self.cont_level_mut().instructions.pop().unwrap();
			self.process_instructions(instr);
		}

		self.handle_next_op();

		Some(Object::default())
	}
}

impl ContextStack {
	fn process_instructions(&mut self, instruction: TopLevelItem) {
		match instruction {
			TopLevelItem::Definition(d) => self.process_instr_definition(d),
			TopLevelItem::Valuable(v) => self.process_instr_valuable(v),
			TopLevelItem::Dropper(value) => self.process_instr_dropper(value),
		}
	}

	fn process_instr_definition(&mut self, definition: Definition) {
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

	fn process_instr_valuable(&mut self, value: Valuable) {
		match value {
			Valuable::Expression(expr) => {
				// self.ops_mut().push(Operations::PopContext);
				self.ops_mut().push(Operations::PushContext(expr));
			},
			Valuable::ObjectLiteral(literal) => {
				self.ops_mut().push(Operations::PushObject(literal));
			},
			Valuable::ObjectConversion(obj_conv) => {
				self.ops_mut().push(Operations::ConvertObject(obj_conv.symbol));
				self.process_instr_valuable(obj_conv.value);
			},
			Valuable::Identifier(ident) => {
				self.ops_mut().push(Operations::PushIdentifier(ident));
			},
			Valuable::Callback(callback) => {
				self.ops_mut().push(Operations::ExCallback {
					method: callback.target.is_some(),
					callback: callback.id,
					parameters: callback.parameters.len(),
				});
			
				if let Some(target) = *callback.target {
					self.process_instr_valuable(target);
				}

				for param in callback.parameters.into_iter() {
					self.process_instr_valuable(param);
				}
			},
		}
	}

	fn process_instr_dropper(&mut self, value: Option<Valuable>) {
		self.ops_mut().push(Operations::PopContext);

		if let Some(value) = value {
			self.process_instr_valuable(value);
		} else {
			self.ops_mut().push(Operations::PushObject(swear_parser::ObjectLiteral::Zip));
		}
	}

	fn push(&mut self, context: ContextLevel) {
		self.stack.push(context.into());
		self.at_root = false;
	}

	fn pop(&mut self) {
		if self.at_root {
			panic!("Cannot pop the root context");
		}

		self.stack.pop();

		if self.stack.len() == 1 {
			self.at_root = true;
		}
	}

	// fn cont(&self) -> &ContextHolder {
	// 	self.stack.last().unwrap()
	// }

	fn cont_mut(&mut self) -> &mut ContextHolder {
		self.stack.last_mut().unwrap()
	}

	fn cont_level(&self) -> &ContextLevel {
		let mut iter = self.stack.iter().rev();
		loop {
			if let ContextHolder::ContextLevel(level) = iter.next().unwrap() {
				return level;
			}
		}
	}

	fn cont_level_mut(&mut self) -> &mut ContextLevel {
		let mut iter = self.stack.iter_mut().rev();
		loop {
			if let ContextHolder::ContextLevel(level) = iter.next().unwrap() {
				return level;
			}
		}
	}

	fn ops(&self) -> &Vec<Operations> {
		&self.cont_level().ops
	}

	fn ops_mut(&mut self) -> &mut Vec<Operations> {
		&mut self.cont_level_mut().ops
	}

	// fn table(&self) -> &Vec<ObjectRef> {
	// 	&self.table
	// }

	fn table_mut(&mut self) -> &mut Vec<ObjectRef> {
		&mut self.table
	}
}

impl Context for ContextStack {
	fn get(&self,key: &str) -> Option<ContextItem> {
		for context in self.stack.iter().rev() {
			if let Some(value) = context.get(key) {
				return Some(value);
			}
		}

		None
	}

	fn set(&mut self, key: String, value: ContextItem) {
		self.cont_mut().set(key, value);
	}
}
