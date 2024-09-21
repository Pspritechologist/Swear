mod context;
mod object_ref;

pub use context::*;
pub use object_ref::*;
use swear_parser::Expression;

use crate::object::*;
use std::{fmt::Debug, ops::{Deref, DerefMut}, sync::{Arc, Mutex, RwLock}};

#[repr(C)] //? Used in Dynamic libraries.
#[derive(Debug, Clone)]
pub enum ContextItem {
	Object(ObjectRef),
	Callback(Callback),
	Blueprint(Blueprint),
}
impl From<ObjectRef> for ContextItem {
	fn from(object: ObjectRef) -> Self {
		Self::Object(object)
	}
}
impl From<Callback> for ContextItem {
	fn from(callback: Callback) -> Self {
		Self::Callback(callback)
	}
}
impl From<Blueprint> for ContextItem {
	fn from(blueprint: Blueprint) -> Self {
		Self::Blueprint(blueprint)
	}
}

#[repr(C)] //? Used in Dynamic libraries.
#[derive(Debug, Clone)]
pub enum Callback {
	Swear(SwearCallback),
	Native(NativeCallback),
}

impl From<SwearCallback> for Callback {
	fn from(callback: SwearCallback) -> Self {
		Self::Swear(callback)
	}
}
impl From<NativeCallback> for Callback {
	fn from(callback: NativeCallback) -> Self {
		Self::Native(callback)
	}
}

#[derive(Debug, Clone)]
pub struct SwearCallback {
	pub args: Vec<String>,
	pub callback: Expression,
}

#[repr(C)] //? Used in Dynamic libraries.
#[derive(Clone)]
pub struct NativeCallback {
	pub arg_count: usize,
	pub callback: Arc<Mutex<ObjectFunction>>,
}

impl Debug for NativeCallback {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "Callback")
	}
}

#[derive(Debug, Clone)]
pub struct Blueprint {
	pub expr: Expression,
}

impl From<Expression> for Blueprint {
	fn from(expr: Expression) -> Self {
		Self { expr }
	}
}
