mod context;
mod object_ref;

pub use context::*;
pub use object_ref::*;
use swear_parser::Expression;

use crate::object::*;
use std::{fmt::Debug, ops::{Deref, DerefMut}, sync::{Arc, Mutex, RwLock}};

#[repr(C)] //? Used in Dynamic libraries.
#[derive(Debug, Clone)]
pub enum ContextItem<'rt> {
	Object(ObjectRef<'rt>),
	Callback(Callback<'rt>),
	Blueprint(Blueprint<'rt>),
}
impl<'rt> From<ObjectRef<'rt>> for ContextItem<'rt> {
	fn from(object: ObjectRef<'rt>) -> Self {
		Self::Object(object)
	}
}
// impl<'rt> From<Callback<'rt>> for ContextItem<'rt> {
// 	fn from(callback: Callback) -> Self {
// 		Self::Callback(callback)
// 	}
// }
// impl<'rt> From<Blueprint> for ContextItem<'rt> {
// 	fn from(blueprint: Blueprint) -> Self {
// 		Self::Blueprint(blueprint)
// 	}
// }

#[repr(C)] //? Used in Dynamic libraries.
#[derive(Debug, Clone)]
pub enum Callback<'rt> {
	Swear(SwearCallback<'rt>),
	Native(NativeCallback<'rt>),
}

// impl<'rt> From<SwearCallback<'rt>> for Callback<'rt> {
// 	fn from(callback: SwearCallback) -> Self {
// 		Self::Swear(callback)
// 	}
// }
impl<'rt> From<NativeCallback<'rt>> for Callback<'rt> {
	fn from(callback: NativeCallback<'rt>) -> Self {
		Self::Native(callback)
	}
}

#[derive(Debug, Clone)]
pub struct SwearCallback<'rt> {
	pub args: Vec<String>,
	pub callback: &'rt Expression,
}

#[repr(C)] //? Used in Dynamic libraries.
#[derive(Clone)]
pub struct NativeCallback<'rt> {
	pub arg_count: usize,
	pub callback: Arc<Mutex<ObjectFunction<'rt>>>,
}

impl<'rt> Debug for NativeCallback<'rt> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "Callback")
	}
}

#[derive(Debug, Clone)]
pub struct Blueprint<'rt> {
	pub expr: &'rt Expression,
}

// impl From<Expression> for Blueprint {
// 	fn from(expr: Expression) -> Self {
// 		Self { expr }
// 	}
// }
