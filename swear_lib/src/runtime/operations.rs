use super::*;
use swear_parser::{Expression, ObjectLiteral, ObjectSymbol};

use self::Operations::*;

use super::ContextStack;

// #[cfg_attr(feature="serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug)]
pub enum Operations<'rt> {
	PushObject(&'rt ObjectLiteral),
	ConvertObject(&'rt ObjectSymbol),
	PushIdentifier(&'rt String),
	RegisterObject(&'rt String),
	RegisterCallback {
		ident: &'rt String,
		parameters: &'rt Vec<String>,
		expr: &'rt Expression,
	},
	RegisterBlueprint {
		ident: &'rt String,
		expr: &'rt Expression,
	},
	ExCallback {
		method: bool,
		callback: &'rt String,
		parameters: usize,
	},
	Repeat(&'rt TopLevelItem),
	PushContext(&'rt Expression),
	PopContext,
}

impl<'rt> ContextStack<'rt> {
	/// Handles the next operation in the stack.
	/// 
	/// # Panics
	/// 
	/// Panics if the operation stack is empty.
	pub fn handle_next_op(&mut self) {
		let operation = self.ops_mut().pop().unwrap();
		self.handle_op(operation);
	}

	pub fn handle_op(&mut self, op: Operations<'rt>) {
		match op {
			PushObject(object) => {
				self.table_mut().push(Object::from_literal(object).into());
			},
			ConvertObject(symbol) => {
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
			PushIdentifier(ident) => {
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
								self.push(ContextLevel::new(&callback).into());
								for arg in args.into_iter().rev() {
									self.set(arg, ObjectRef::default().into());
								}
							}
						}
					},
					Some(ContextItem::Blueprint(blueprint)) => {
						self.push(BlueprintContext::new(blueprint.expr).into());
					},
					None => self.table_mut().push(Object::default().into()),
				}
				
			},
			ExCallback { method, callback: id, parameters } => {
				let (obj, Some(callback)) = (if method {
					let objref = self.table_pop();
					let obj = objref.access();
					let funcs = obj.get_functions();
					let func = funcs.get(id);
					drop(obj);
					(Some(objref), func.map(|func| func.function.clone()))
				} else {
					match self.get(&id) {
						Some(ContextItem::Callback(callback)) => (None, Some(callback.clone())),
						_ => (None, None),
					}
				}) else {
					println!("Function not found: {}", id);
					self.table_mut().push(Object::default().into());
					return;
				};

				match callback {
					Callback::Native(callback) => {
						let mut args = Vec::with_capacity(parameters);
						for _ in 0..parameters {
							args.push(self.table_pop());
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
						self.push(ContextLevel::new(&callback.callback).into());
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
							let obj = self.table_pop();
							self.set(arg, obj.into());
						}
					}
				}
			},
			Repeat(instr) => {
				let cond = self.table_pop();
				let cond = cond.access();
				let count = match &*cond {
					Object::State(state) => if state.state { 1 } else { 0 },
					Object::Count(count) => count.count.to_f32().value() as usize,
					obj => obj.to_count().count.to_f32().value() as usize,
				};

				for _ in 0..count {
					self.process_instructions(instr);
				}
			},
			RegisterObject(ident) => {
				let obj = self.table_pop();
				self.set(ident.clone(), obj.into()); //TODO: Clone
			},
			RegisterCallback { ident, parameters, expr } => {
				self.set(ident.clone(), ContextItem::Callback(Callback::Swear(SwearCallback { //TODO: Clone
					args: parameters.clone(), //TODO: Clone
					callback: expr,
				})));
			},
			RegisterBlueprint { ident, expr } => {
				self.set(ident.clone(), ContextItem::Blueprint(Blueprint {
					expr,
				}));
			},
			PushContext(instructions) => {
				self.push(ContextLevel::new(instructions).into());
			},
			PopContext => {
				self.pop();
			},
		}
	}
}
