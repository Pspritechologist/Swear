use super::*;
use swear_parser::{Expression, ObjectLiteral, ObjectSymbol};

use super::ContextStack;

#[cfg_attr(feature="serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug)]
pub enum Operations {
	PushObject(ObjectLiteral),
	ConvertObject(ObjectSymbol),
	PushIdentifier(String),
	RegisterObject(String),
	RegisterCallback {
		ident: String,
		parameters: Vec<String>,
		expr: Expression,
	},
	RegisterBlueprint {
		ident: String,
		expr: Expression,
	},
	ExCallback {
		method: bool,
		callback: String,
		parameters: usize,
	},
	PushContext(Expression),
	PopContext,
	ConstructDynamicObject,
}

impl ContextStack {
	/// Handles the next operation in the stack.
	/// 
	/// # Panics
	/// 
	/// Panics if the operation stack is empty.
	pub fn handle_next_op(&mut self) {
		let operation = self.ops_mut().pop().unwrap();
		self.handle_op(operation);
	}

	pub fn handle_op(&mut self, op: Operations) {
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
					return;
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
	}
}
