use std::hash::Hash;

use super::*;

#[derive(Clone)]
pub struct ObjectRef {
	inner: Arc<RwLock<Object>>,
}

impl PartialEq for ObjectRef {
	fn eq(&self, other: &Self) -> bool {
		*self.inner.read().unwrap() == *other.inner.read().unwrap()
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

impl Deref for ObjectRef {
	type Target = Arc<RwLock<Object>>;
	
	fn deref(&self) -> &Self::Target {
		&self.inner
	}
}

impl DerefMut for ObjectRef {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.inner
	}
}
