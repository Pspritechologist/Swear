use crate::runtime::ObjectRef;

use super::*;

#[derive(Clone, Default, PartialEq, Eq)]
pub struct Map {
	pub map: Vec<(ObjectRef, ObjectRef)>,
}

impl Map {
	pub fn from_iter_ref<I: IntoIterator<Item = (ObjectRef, ObjectRef)>>(iter: I) -> Self {
		Self { map: iter.into_iter().collect() }
	}

	pub fn from_iter_obj<I: IntoIterator<Item = (Object, Object)>>(iter: I) -> Self {
		Self { map: iter.into_iter().map(|(k, v)| (ObjectRef::from(k), ObjectRef::from(v))).collect() }
	}

	pub fn from_iter_lit<I: IntoIterator<Item = (ObjectLiteral, ObjectLiteral)>>(iter: I) -> Self {
		Self { map: iter.into_iter().map(|(k, v)| (Object::from(k).into(), Object::from(v).into())).collect() }
	}
}

impl IObject for Map {
	fn to_chars(&self) -> Chars {
		let mut chars = String::new();
		let last = self.map.last();

		fn item(dest: &mut String, k: &ObjectRef, v: &ObjectRef) {
			dest.push_str(&k.read().unwrap().to_chars().chars); //FIXME Use to_string()
			dest.push_str(": ");
			dest.push_str(&v.read().unwrap().to_chars().chars); //FIXME Use to_string()
		}

		chars.push('{');
		for (k, v) in self.map.iter().take(self.map.len() - 1) {
			item(&mut chars, k, v);
			chars.push_str(", ");
		}
		if let Some((k, v)) = last {
			item(&mut chars, k, v);
		}
		chars.push('}');
		chars.into()
	}

	fn to_count(&self) -> Count {
		self.map.len().into()
	}

	fn to_state(&self) -> State {
		(!self.map.is_empty()).into()
	}

	fn to_deck(&self) -> Deck {
		Deck::default()	
	}

	fn to_map(&self) -> Map {
		self.clone()
	}

	fn object_name(&self) ->  &str {
		"Map"
	}
}

impl std::fmt::Debug for Map {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		// Ensure formatter flags are used correctly
		let mut debug = f.debug_map();
		for (k, v) in self.map.iter() {
			debug.entry(&k, &v);
		}
		debug.finish()
	}
}

impl From<Vec<(ObjectRef, ObjectRef)>> for Map {
	fn from(map: Vec<(ObjectRef, ObjectRef)>) -> Self {
		Self { map }
	}
}

impl From<Vec<(ObjectRef, Object)>> for Map {
	fn from(map: Vec<(ObjectRef, Object)>) -> Self {
		Self { map: map.into_iter().map(|(k, v)| (k, ObjectRef::from(v))).collect() }
	}
}

impl From<Vec<(Object, ObjectRef)>> for Map {
	fn from(map: Vec<(Object, ObjectRef)>) -> Self {
		Self { map: map.into_iter().map(|(k, v)| (ObjectRef::from(k), v)).collect() }
	}
}

impl From<Vec<(Object, Object)>> for Map {
	fn from(map: Vec<(Object, Object)>) -> Self {
		Self { map: map.into_iter().map(|(k, v)| (ObjectRef::from(k), ObjectRef::from(v))).collect() }
	}
}

impl FromIterator<(ObjectRef, ObjectRef)> for Map {
	fn from_iter<T: IntoIterator<Item = (ObjectRef, ObjectRef)>>(iter: T) -> Self {
		Self { map: iter.into_iter().collect() }
	}
}

impl FromIterator<(ObjectRef, Object)> for Map {
	fn from_iter<T: IntoIterator<Item = (ObjectRef, Object)>>(iter: T) -> Self {
		Self { map: iter.into_iter().map(|(k, v)| (k, ObjectRef::from(v))).collect() }
	}
}

impl FromIterator<(Object, ObjectRef)> for Map {
	fn from_iter<T: IntoIterator<Item = (Object, ObjectRef)>>(iter: T) -> Self {
		Self { map: iter.into_iter().map(|(k, v)| (ObjectRef::from(k), v)).collect() }
	}
}

impl FromIterator<(Object, Object)> for Map {
	fn from_iter<T: IntoIterator<Item = (Object, Object)>>(iter: T) -> Self {
		Self { map: iter.into_iter().map(|(k, v)| (ObjectRef::from(k), ObjectRef::from(v))).collect() }
	}
}
