pub mod operations;

pub use crate::context::ObjectRef;

use crate::object::*;
use crate::context::*;
use operations::Operations;
use swear_parser::Expression;
use swear_parser::ObjectSymbol;
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

		let op = self.ops_mut().pop().unwrap();
		match op {
			Operations::PushObject(object) => {
				self.table_mut().push(Object::from_literal(object).into());
			},
			Operations::ConvertObject(symbol) => {
				let object = self.table_mut().pop().expect("FIXME");
				let object = object.access();
				self.table_mut().push(match symbol {
					ObjectSymbol::Chars => Object::from(object.to_chars()).into(),
					ObjectSymbol::Count => Object::from(object.to_count()).into(),
					ObjectSymbol::State => Object::from(object.to_state()).into(),
					ObjectSymbol::Zip => Object::from(object.to_zip()).into(),
					ObjectSymbol::Deck => Object::from(object.to_deck()).into(),
					ObjectSymbol::Map => Object::from(object.to_map()).into(),
				});
			},
			Operations::PushIdentifier(ident) => {
				let item = self.get(&ident);
				match item {
					Some(ContextItem::Object(obj)) => {
						let obj = obj.copy();
						self.table_mut().push(obj);
					},
					Some(ContextItem::Callback(callback)) => {
						match callback {
							Callback::Native(NativeCallback { arg_count: _, callback: _ }) => {
								// let mut args = Vec::with_capacity(arg_count);
								// for _ in 0..arg_count {
								// 	args.push(Object::default().into()); //FIXME: Arg count.
								// }
		
								// let result = callback.lock().unwrap().call_mut((None, args)).ok().flatten().unwrap_or_default(); //TODO: Error handling.
								// self.table_mut().push(result);
								unreachable!("Called non-method native function"); //? Swear does not have native functions.
							},
							Callback::Swear(SwearCallback { args, callback }) => {
								self.push(ContextLevel::new(callback));
								for arg in args.into_iter().rev() {
									self.set(arg, ObjectRef::default().into());
								}
							}
						}
					},
					Some(ContextItem::Blueprint(_blueprint)) => {
						// self.push(ContextLevel::new(blueprint.callback.clone()));
						todo!()
					},
					None => self.table_mut().push(Object::default().into()),
				}
				
			},
			Operations::ExCallback { method, callback: id, parameters } => {
				let (obj, Some(callback)) = (if method {
					let objref = self.table_mut().pop().unwrap();
					let obj = objref.access();
					let func = obj.get_functions().get(&id).map(|info| info.function.clone());
					drop(obj);
					(Some(objref), func)
				} else {
					match self.get(&id) {
						Some(ContextItem::Callback(callback)) => (None, Some(callback)),
						_ => (None, None),
					}
				}) else {
					println!("Function not found: {}", id);
					self.table_mut().push(Object::default().into());
					return None;
				};

				match callback {
					Callback::Native(callback) => {
						let mut args = Vec::with_capacity(parameters);
						for _ in 0..parameters {
							args.push(self.table_mut().pop().unwrap());
						}

						let result = callback.callback
							.lock()
							.unwrap()
							.call_mut((obj.expect("Called non-method native function"), args))
							.ok()
							.flatten()
							.unwrap_or_default(); //TODO: Error handling.

						self.table_mut().push(result);
					},
					Callback::Swear(callback) => {
						self.push(ContextLevel::new(callback.callback));
						let diff = parameters as i128 - callback.args.len() as i128; //? i128 to prevent underflow.
						if diff > 0 {
							for _ in 0..=diff {
								self.table_mut().pop();
							}
						} else if diff < 0 {
							for _ in 0..=diff {
								self.table_mut().push(Object::default().into());
							}
						}
						
						for arg in callback.args.into_iter().rev() {
							let obj = self.table_mut().pop().unwrap();
							self.set(arg, obj.into());
						}
					}
				}
			},
			Operations::RegisterObject(ident) => {
				let obj = self.table_mut().pop().unwrap();
				self.set(ident, obj.into());
			},
			Operations::RegisterCallback { ident, parameters, expr } => {
				self.set(ident, Callback::from(SwearCallback {
					args: parameters,
					callback: expr,
				}).into());
			},
			Operations::RegisterBlueprint { ident: _, expr: _ } => {
				// self.set(ident, Blueprint::from(expr).into());
				todo!()
			},
			Operations::PushContext(instructions) => {
				self.push(ContextLevel::new(instructions.into()));
			},
			Operations::PopContext => {
				self.pop();
			},
			Operations::ConstructDynamicObject => {
				todo!()
			}
		}

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
