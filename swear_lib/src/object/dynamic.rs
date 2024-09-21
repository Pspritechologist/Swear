use crate::context::{self, ContextItem};

use super::*;

#[derive(Default)]
#[repr(C)] //? The main point of this is to allow adding new fields without breaking ABI.
#[non_exhaustive]
pub struct Dynamic {
	name: String,
	description: Option<String>,
	contents: std::collections::HashMap<String, ContextItem>,
	//? It's very important we ensure whatever library this Object
	//? interacts with remains loaded until the Object no longer exists.
	#[allow(unused)]
	pub(crate) src_lib: Option<Arc<libloading::Library>>,
}

impl Dynamic {
	pub fn add_function(&mut self, info: FunctionInfo) {
		self.contents.insert(info.name, info.function.into());
	}
}

impl IObject for Dynamic {
	fn to_chars(&self) -> Chars {
		Chars::default()
	}

	fn to_count(&self) -> Count {
		Count::default()
	}

	fn to_state(&self) -> State {
		State::default()
	}

	fn to_deck(&self) -> Deck {
		Deck::default()
	}

	fn to_map(&self) -> Map {
		Map::default()
	}

	fn is_dynamic(&self) -> bool {
		true
	}

	fn get_info(&self) -> ObjectInfo {
		ObjectInfo::from_str(&self.name)
			.with_description(self.description.clone())
	}

	fn get_functions(&self) -> HashMap<String, FunctionInfo> {
		self.contents.iter().filter_map(|(key, value)| match value {
			ContextItem::Callback(Callback::Native(NativeCallback { callback, .. })) => {
				Some((key.clone(), FunctionInfoBuilder::new(key.clone()).build(callback.clone())))
			},
			_ => None,
		}).collect()
	}
}

impl context::Context for Dynamic {
	fn get(&self, key: &str) -> Option<ContextItem> {
		self.contents.get(key).cloned()
	}

	fn set(&mut self, key: String, value: ContextItem) {
		self.contents.insert(key, value);
	}
}

impl std::fmt::Debug for Dynamic {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "Dynamic({:?})", self.contents)
	}
}

#[cfg(feature="serde")]
mod serde_impl {
	// Simply write a 'Zip' instead.
	// Read as a blank Dynamic object.
	use super::*;
	use serde::{Deserialize, Deserializer, Serialize, Serializer};

	impl Serialize for Dynamic {
		fn serialize<S: Serializer>(&self, _serializer: S) -> Result<S::Ok, S::Error> {
			_serializer.serialize_unit()
		}
	}

	impl<'de> Deserialize<'de> for Dynamic {
		fn deserialize<D: Deserializer<'de>>(_deserializer: D) -> Result<Self, D::Error> {
			Ok(Dynamic::default())
		}
	}
}
