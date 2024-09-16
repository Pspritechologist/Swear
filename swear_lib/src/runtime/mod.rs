pub mod operations;

pub use crate::context::ObjectRef;

use crate::object::*;
use crate::context::*;
use operations::Operations;
use swear_parser::{Definition, ObjectSymbol, TopLevelItem, Valuable};

#[derive(Debug)]
pub struct ContextStack {
	stack: Vec<ContextLevel>,
	at_root: bool,
}

pub trait SwearRuntime {
	fn new(script: Vec<TopLevelItem>) -> Self;
	fn step(&mut self) -> Option<Object>;
}

impl SwearRuntime for ContextStack {
	fn new(script: Vec<TopLevelItem>) -> Self {
		let mut stack = Self {
			stack: Vec::new(),
			at_root: true,
		};

		stack.push(ContextLevel::new(script));
		stack
	}

	fn step(&mut self) -> Option<Object> {
		while self.ops().is_empty() {
			if self.cont().instr_index >= self.cont().instructions.len() {
				if !self.at_root {
					let obj = self.table_mut().pop().unwrap_or_else(|| Object::default().into());
					self.pop();
					self.table_mut().push(obj);
					continue;
				} else {
					return None;
				}
			}

			self.derive_operations(&self.cont().instructions[self.cont().instr_index].clone()); //FIXME
			self.cont_mut().instr_index += 1;
		}
		
		let operation = self.ops_mut().pop().unwrap();
		match operation {
			Operations::PushObject(object) => {
				self.table_mut().push(Object::from_literal(object).into());
			},
			Operations::ConvertObject(symbol) => {
				let object = self.table_mut().pop().expect("FIXME");
				let object = object.read().unwrap();
				self.table_mut().push(match symbol {
					ObjectSymbol::Chars => Object::from(object.to_chars()).into(),
					ObjectSymbol::Count => Object::from(object.to_count()).into(),
					ObjectSymbol::State => Object::from(object.to_state()).into(),
					ObjectSymbol::Zip => Object::from(object.to_zip()).into(),
					ObjectSymbol::Deck => Object::from(object.to_deck()).into(),
					ObjectSymbol::Map => Object::from(object.to_map()).into(),
				});
			},
			Operations::PushContext(instructions) => {
				self.push(ContextLevel::new(instructions));
			},
			Operations::PopContext => {
				self.pop();
			},
			Operations::PushIdentifier(ident) => {
				let obj = self.get(&ident).unwrap_or_else(|| Object::default().into());
				self.table_mut().push(obj);
			},
			Operations::CallCallback { method, callback, parameters } => {
				if callback == "scribe" {
					if !method {
						panic!("'scribe' is a method");
					}
					if parameters != 0 {
						panic!("'scribe' takes no parameters");
					}
					
					let target = self.table_mut().pop().unwrap();
					let target = target.read().unwrap();

					println!("{}", target.to_chars().chars);

					self.table_mut().push(Object::default().into());
				} else {
					todo!()
				}
			},
			Operations::RegisterObject(ident) => {
				let obj = self.table_mut().pop().unwrap();
				self.set(ident, obj);
			},
			Operations::RegisterCallback(ident) => {
				todo!()
			},
			Operations::RegisterBlueprint(ident) => {
				todo!()
			},
		}

		Some(Object::default())
	}
}

impl ContextStack {
	fn derive_operations(&mut self, instruction: &TopLevelItem) {
		match instruction {
			TopLevelItem::Definition(d) => self.derive_operations_definition(d),
			TopLevelItem::Valuable(v) => self.derive_operations_value(v),
			TopLevelItem::Dropper(value) => self.derive_operations_dropper(value),
		}
	}

	fn derive_operations_definition(&mut self, definition: &Definition) {
		match definition {
			Definition::Blueprint { name, exprs } => todo!(),
			Definition::Callback { name, parameters, exprs } => todo!(),
			Definition::Register { name, value } => {
				self.ops_mut().push(Operations::RegisterObject(name.clone()));
				self.derive_operations_value(value);
			},
		}
	}

	fn derive_operations_value(&mut self, value: &Valuable) {
		match value {
			Valuable::Expression(expr) => {
				// self.ops_mut().push(Operations::PopContext);
				self.ops_mut().push(Operations::PushContext(expr.contents.clone()));
			},
			Valuable::ObjectLiteral(literal) => {
				self.ops_mut().push(Operations::PushObject(literal.clone()));
			},
			Valuable::ObjectConversion(obj_conv) => {
				self.ops_mut().push(Operations::ConvertObject(obj_conv.symbol));
				self.derive_operations_value(&obj_conv.value);
			},
			Valuable::Identifier(ident) => {
				self.ops_mut().push(Operations::PushIdentifier(ident.clone()));
			},
			Valuable::Callback(callback) => {
				self.ops_mut().push(Operations::CallCallback {
					method: callback.target.is_some(),
					callback: callback.id.clone(),
					parameters: callback.parameters.len(),
				});

				for param in callback.parameters.iter() {
					self.derive_operations_value(param);
				}
			
				if let Some(target) = callback.target.as_ref() {
					self.derive_operations_value(target);
				}
			},
		}
	}

	fn derive_operations_dropper(&mut self, value: &Option<Valuable>) {
		self.ops_mut().push(Operations::PopContext);

		if let Some(value) = value {
			self.derive_operations_value(value);
		} else {
			self.ops_mut().push(Operations::PushObject(swear_parser::ObjectLiteral::Zip));
		}
	}

	fn push(&mut self, context: ContextLevel) {
		self.stack.push(context);
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

	fn cont(&self) -> &ContextLevel {
		self.stack.last().unwrap()
	}

	fn cont_mut(&mut self) -> &mut ContextLevel {
		self.stack.last_mut().unwrap()
	}

	fn ops(&self) -> &Vec<Operations> {
		&self.cont().ops
	}

	fn ops_mut(&mut self) -> &mut Vec<Operations> {
		&mut self.cont_mut().ops
	}

	fn table(&self) -> &Vec<ObjectRef> {
		&self.cont().table
	}

	fn table_mut(&mut self) -> &mut Vec<ObjectRef> {
		&mut self.cont_mut().table
	}

	fn get(&self, key: &str) -> Option<ObjectRef> {
		for context in self.stack.iter().rev() {
			if let Some(value) = context.get(key) {
				return Some(value);
			}
		}

		None
	}

	fn set(&mut self, key: String, value: ObjectRef) {
		self.stack.last_mut().unwrap().set(key, value);
	}
}
