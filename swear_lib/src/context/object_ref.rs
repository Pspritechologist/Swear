use std::hash::Hash;

use super::*;

#[repr(C)] //? Used in Dynamic libraries.
#[derive(Clone, Default)]
pub struct ObjectRef {
	inner: Arc<RwLock<Object>>,
}

impl PartialEq for ObjectRef {
	fn eq(&self, other: &Self) -> bool {
		*self.access() == *other.access()
	}
}

impl Eq for ObjectRef {}

impl Hash for ObjectRef {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		Arc::as_ptr(&self.inner).hash(state)
	}
}

impl ObjectRef {
	pub fn new(inner: Object) -> Self {
		Self {
			inner: Arc::new(RwLock::new(inner)),
		}
	}

	pub fn access(&self) -> std::sync::RwLockReadGuard<Object> {
		match self.inner.try_read() {
			Ok(guard) => guard,
			Err(std::sync::TryLockError::WouldBlock) => dbg!(self).inner.read().unwrap(),
			Err(err) => panic!("Failed to lock ObjectRef: {:?}", dbg!(err)),
		}
	}

	pub fn lock(&self) -> std::sync::RwLockWriteGuard<Object> {
		match self.inner.try_write() {
			Ok(guard) => guard,
			Err(std::sync::TryLockError::WouldBlock) => dbg!(self).inner.write().unwrap(),
			Err(err) => panic!("Failed to lock ObjectRef mutably: {:?}", dbg!(err)),
		}
	}
}

impl std::fmt::Debug for ObjectRef {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		if let Ok(inner) = self.inner.read() {
			inner.fmt(f)
		} else {
			write!(f, "ObjectRef(LOCKED)")
		}
	}
}

impl From<Object> for ObjectRef {
	fn from(inner: Object) -> Self {
		Self::new(inner)
	}
}

// impl Deref for ObjectRef {
// 	type Target = Arc<RwLock<Object>>;
	
// 	fn deref(&self) -> &Self::Target {
// 		&self.inner
// 	}
// }

// impl DerefMut for ObjectRef {
// 	fn deref_mut(&mut self) -> &mut Self::Target {
// 		&mut self.inner
// 	}
// }

impl Context for ObjectRef {
	fn get(&self, key: &str) -> Option<ContextItem> {
		let lock = self.access();
		match lock.deref() {
			Object::Dynamic(obj) => obj.get(key).clone(),
			other => {
				dbg!(other);
				None
			},
		}
	}

	fn set(&mut self, key: String, value: ContextItem) {
		let mut lock = self.lock();
		match lock.deref_mut() {
			Object::Dynamic(obj) => obj.set(key, value),
			other => {
				dbg!(other);
			},
		}
	}
}

#[cfg(feature="serde")]
mod serde_impl {
	use super::*;
	use serde::{Serialize, Deserialize, Serializer, Deserializer};

	impl Serialize for ObjectRef {
		fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
			self.access().deref().serialize(serializer)
		}
	}

	impl<'de> Deserialize<'de> for ObjectRef {
		fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
			Object::deserialize(deserializer).map(ObjectRef::from)
		}
	}
}
