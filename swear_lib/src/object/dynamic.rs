use crate::context::{self, ContextItem};

use super::*;

#[derive(Default)]
#[repr(C)] //? The main point of this is to allow adding new fields without breaking ABI.
#[non_exhaustive]
pub struct Dynamic<'rt> {
	name: String,
	description: Option<String>,
	contents: std::collections::HashMap<String, ContextItem<'rt>>,
	//? It's very important we ensure whatever library this Object
	//? interacts with remains loaded until the Object no longer exists.
	#[allow(unused)]
	pub(crate) src_lib: Option<Arc<libloading::Library>>,
}

impl<'rt> Dynamic<'rt> {
	pub fn add_function(&mut self, info: FunctionInfo<'rt>) {
		self.contents.insert(info.name, ContextItem::Callback(info.function));
	}
}

impl<'rt> IObject<'rt> for Dynamic<'rt> {
	fn to_chars(&self) -> Chars {
		Chars::default()
	}

	fn to_count(&self) -> Count {
		Count::default()
	}

	fn to_state(&self) -> State {
		State::default()
	}

	fn to_deck(&self) -> Deck<'rt> {
		Deck::default()
	}

	fn to_map(&self) -> Map<'rt> {
		Map::default()
	}

	fn is_dynamic(&self) -> bool {
		true
	}

	fn get_info(&self) -> ObjectInfo {
		ObjectInfo::from_str(&self.name)
			.with_description(self.description.clone())
	}

	fn get_functions(&self) -> HashMap<String, FunctionInfo<'rt>> {
		self.contents.iter().filter_map(|(key, value)| match value {
			ContextItem::Callback(Callback::Native(NativeCallback { callback, .. })) => {
				Some((key.clone(), FunctionInfoBuilder::new(key.clone()).build(callback.clone())))
			},
			_ => None,
		}).collect()
	}
}

impl<'rt> context::Context<'rt> for Dynamic<'rt> {
	fn get(&self, key: &str) -> Option<ContextItem<'rt>> {
		self.contents.get(key).cloned()
	}

	fn set(&mut self, key: String, value: ContextItem<'rt>) {
		self.contents.insert(key, value);
	}
}

impl<'rt> std::fmt::Debug for Dynamic<'rt> {
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

	impl<'rt> Serialize for Dynamic<'rt> {
		fn serialize<S: Serializer>(&self, _serializer: S) -> Result<S::Ok, S::Error> {
			_serializer.serialize_unit()
		}
	}

	impl<'de, 'rt> Deserialize<'de> for Dynamic<'rt> {
		fn deserialize<D: Deserializer<'de>>(_deserializer: D) -> Result<Self, D::Error> {
			Ok(Dynamic::default())
		}
	}
}
