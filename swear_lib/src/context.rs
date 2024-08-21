use std::rc::Rc;

use super::*;

#[derive(Debug, Clone)]
pub enum ContextVar {
	Register(RefCell<Object>),
	Blueprint(Operations),
	Callback(Callback),
}

impl From<Object> for ContextVar {
	fn from(obj: Object) -> Self {
		ContextVar::Register(RefCell::new(obj))
	}
}
impl From<Operations> for ContextVar {
	fn from(expr: Operations) -> Self {
		ContextVar::Blueprint(expr)
	}
}
impl From<Callback> for ContextVar {
	fn from(cb: Callback) -> Self {
		ContextVar::Callback(cb)
	}
}

pub trait Context {
	fn get_value(&self, key: &str) -> ContextVar;
	fn has_value(&self, key: &str) -> bool;
	fn set_value(&mut self, key: &str, value: ContextVar);

	// fn get_result(input: &mut ContextVar) -> RefCell<Object> {
	// 	match input {
	// 		ContextVar::Register(obj) => obj.clone(),
	// 		ContextVar::Blueprint(expr) => todo!(),
	// 		ContextVar::Callback(cb) => todo!(),
	// 	}
	// }
}

#[derive(Debug, Clone)]
pub struct RootContext {
	pub registers: Rc<RefCell<HashMap<String, ContextVar>>>,
}

impl RootContext {
	pub fn new() -> Self {
		Self {
			registers: Rc::new(RefCell::new(HashMap::new())),
		}
	}
}

impl Context for RootContext {
	fn get_value(&self, key: &str) -> ContextVar {
		// self.registers.entry(key.into()).or_insert_with(|| Object::default().into())
		self.registers.borrow().get(key).unwrap_or(&Object::default().into()).clone()
	}

	fn has_value(&self, key: &str) -> bool {
		self.registers.borrow().contains_key(key)
	}

	fn set_value(&mut self, key: &str, value: ContextVar) {
		self.registers.borrow_mut().insert(key.into(), value);
	}
}

#[derive(Debug, Clone)]
pub struct SubContext<P>
where P: Context + Debug {
	pub parent: P,
	pub registers: Rc<RefCell<HashMap<String, ContextVar>>>,
}

impl<P> SubContext<P>
where P: Context + Debug {
	pub fn new(parent: P) -> Self {
		Self {
			parent,
			registers: Rc::new(RefCell::new(HashMap::new())),
		}
	}
}

impl<'a, P> Context for SubContext<P>
where P: Context + Debug {
	fn get_value(&self, key: &str) -> ContextVar {
		if self.registers.borrow().contains_key(key) {
			self.registers.borrow().get(key).unwrap().clone()
		} else if self.parent.has_value(key) {
			self.parent.get_value(key)
		} else {
			// self.registers.insert(key.into(), Object::default().into());
			// self.registers.get_mut(key).unwrap()
			Object::default().into()
		}
	}

	fn has_value(&self, key: &str) -> bool {
		self.registers.borrow().contains_key(key) || self.parent.has_value(key)
	}

	fn set_value(&mut self, key: &str, value: ContextVar) {
		self.registers.borrow_mut().insert(key.into(), value);
	}
}

struct InnerContext {
	registers: HashMap<String, ContextVar>,
}
