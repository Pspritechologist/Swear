use std::hash::Hash;

use super::*;

#[repr(C)] //? Used in Dynamic libraries.
#[derive(Clone, Default)]
pub struct ObjectRef<'rt> {
	inner: Arc<RwLock<Object<'rt>>>,
}

impl<'rt> IntoIterator for ObjectRef<'rt> {
	type Item = (String, ContextItem<'rt>);
	type IntoIter = std::collections::hash_map::IntoIter<String, ContextItem<'rt>>;
	fn into_iter(self) -> Self::IntoIter {
		match self.access().deref() {
			Object::Dynamic(obj) => obj.clone().into_iter(),
			other => {
				dbg!(other);
				panic!();
			},
		}
	}
}

impl<'rt> PartialEq for ObjectRef<'rt> {
	fn eq(&self, other: &Self) -> bool {
		*self.access() == *other.access()
	}
}

impl<'rt> Eq for ObjectRef<'rt> {}

impl<'rt> Hash for ObjectRef<'rt> {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		Arc::as_ptr(&self.inner).hash(state)
	}
}

impl<'rt> ObjectRef<'rt> {
	pub fn new(inner: Object<'rt>) -> Self {
		Self {
			inner: Arc::new(RwLock::new(inner)),
		}
	}

	pub fn access(&self) -> std::sync::RwLockReadGuard<Object<'rt>> {
		match self.inner.try_read() {
			Ok(guard) => guard,
			Err(std::sync::TryLockError::WouldBlock) => dbg!(self).inner.read().unwrap(),
			Err(err) => panic!("Failed to lock ObjectRef: {:?}", dbg!(err)),
		}
	}

	pub fn lock(&self) -> std::sync::RwLockWriteGuard<Object<'rt>> {
		match self.inner.try_write() {
			Ok(guard) => guard,
			Err(std::sync::TryLockError::WouldBlock) => dbg!(self).inner.write().unwrap(),
			Err(err) => panic!("Failed to lock ObjectRef mutably: {:?}", dbg!(err)),
		}
	}

	/// Identical to cloning, but doesn't look like cloning.
	#[inline(always)]
	pub fn copy(&self) -> Self {
		self.clone()
	}

	pub(crate) fn inner(self) -> Arc<RwLock<Object<'rt>>> {
		self.inner
	}
}

impl<'rt> std::fmt::Debug for ObjectRef<'rt> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		if let Ok(inner) = self.inner.read() {
			inner.fmt(f)
		} else {
			write!(f, "ObjectRef(LOCKED)")
		}
	}
}

impl<'rt> From<Object<'rt>> for ObjectRef<'rt> {
	fn from(inner: Object<'rt>) -> Self {
		Self::new(inner)
	}
}

// impl<'rt> Deref for ObjectRef<'rt> {
// 	type Target = Arc<RwLock<Object<'rt>>>;
	
// 	fn deref(&self) -> &Self::Target {
// 		&self.inner
// 	}
// }

// impl<'rt> DerefMut for ObjectRef<'rt> {
// 	fn deref_mut(&mut self) -> &mut Self::Target {
// 		&mut self.inner
// 	}
// }

impl<'rt> IContext<'rt> for ObjectRef<'rt> {
	fn get(&self, key: &str) -> Option<ContextItem<'rt>> {
		let lock = self.access();
		match lock.deref() {
			Object::Dynamic(obj) => obj.get(key),
			other => {
				dbg!(other);
				None
			},
		}
	}

	fn set(&mut self, key: String, value: ContextItem<'rt>) {
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

	impl<'rt> Serialize for ObjectRef<'rt> {
		fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
			self.access().deref().serialize(serializer)
		}
	}

	impl<'de, 'rt> Deserialize<'de> for ObjectRef<'rt> {
		fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
			Object::deserialize(deserializer).map(ObjectRef::from)
		}
	}
}
