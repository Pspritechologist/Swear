use crate::runtime::ObjectRef;
use super::*;

#[cfg_attr(feature="serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Default, PartialEq, Eq)]
#[swear_object]
pub struct Map<'rt> {
	pub map: Vec<(ObjectRef<'rt>, ObjectRef<'rt>)>,
}

impl<'rt> Map<'rt> {
	pub fn from_vec_lit(v: &Vec<(ObjectLiteral, ObjectLiteral)>) -> Self {
		Self { map: v.iter().map(|(k, v)| (Object::from_literal(k).into(), Object::from_literal(v).into())).collect() }
	}

	// pub fn from_iter_ref<I: IntoIterator<Item = (ObjectRef, ObjectRef)>>(iter: I) -> Self {
	// 	Self { map: iter.into_iter().collect() }
	// }

	// pub fn from_iter_obj<I: IntoIterator<Item = (Object, Object)>>(iter: I) -> Self {
	// 	Self { map: iter.into_iter().map(|(k, v)| (ObjectRef::from(k), ObjectRef::from(v))).collect() }
	// }

	// pub fn from_iter_lit<I: IntoIterator<Item = (ObjectLiteral, ObjectLiteral)>>(iter: I) -> Self {
	// 	Self { map: iter.into_iter().map(|(k, v)| (Object::from_literal(&k).into(), Object::from_literal(&v).into())).collect() }
	// }
}

impl<'rt> Map<'rt> {
	fn to_swear_chars(&self) -> Chars {
		let mut chars = String::new();

		let mut iter = self.map.iter();
		while let Some((k, v)) = iter.next() {
			chars.push_str(&v.access().to_chars().chars); //FIXME Use to_string()
			chars.push('@');
			chars.push_str(&k.access().to_chars().chars); //FIXME Use to_string()
			chars.push('\n'); //? Attempts to remove this trailing new line have failed.
		}

		chars.into()
	}

	fn to_swear_count(&self) -> Count {
		self.map.len().into()
	}

	fn to_swear_state(&self) -> State {
		(!self.map.is_empty()).into()
	}

	fn to_swear_deck(&self) -> Deck<'rt> {
		Deck::default()	
	}

	fn to_swear_map(&self) -> Map<'rt> {
		self.clone()
	}

	fn get_function(&self, name: &str) -> Option<FunctionInfo<'rt>> {
		Some(match name {
			// Lest function.
			// No op, returns this Object.
			"lest" => 
				FunctionInfoBuilder::new("lest".to_string()).build_native(Arc::new(Mutex::new(|obj, _| Ok(Some(obj))))),

			// Solid function.
			// Returns false if Zip.
			"solid" => 
				FunctionInfoBuilder::new("solid".to_string()).build_native(Arc::new(Mutex::new(|_, _| Ok(Some(Object::from(State::from(true)).into()))))),

			_ => return None,
		})
	}
}

impl<'rt> std::fmt::Debug for Map<'rt> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		// Ensure formatter flags are used correctly
		let mut debug = f.debug_map();
		for (k, v) in self.map.iter() {
			debug.entry(&k, &v);
		}
		debug.finish()
	}
}

impl<'rt> From<Vec<(ObjectRef<'rt>, ObjectRef<'rt>)>> for Map<'rt> {
	fn from(map: Vec<(ObjectRef<'rt>, ObjectRef<'rt>)>) -> Self {
		Self { map }
	}
}

impl<'rt> From<Vec<(ObjectRef<'rt>, Object<'rt>)>> for Map<'rt> {
	fn from(map: Vec<(ObjectRef<'rt>, Object<'rt>)>) -> Self {
		Self { map: map.into_iter().map(|(k, v)| (k, ObjectRef::from(v))).collect() }
	}
}

impl<'rt> From<Vec<(Object<'rt>, ObjectRef<'rt>)>> for Map<'rt> {
	fn from(map: Vec<(Object<'rt>, ObjectRef<'rt>)>) -> Self {
		Self { map: map.into_iter().map(|(k, v)| (ObjectRef::from(k), v)).collect() }
	}
}

impl<'rt> From<Vec<(Object<'rt>, Object<'rt>)>> for Map<'rt> {
	fn from(map: Vec<(Object<'rt>, Object<'rt>)>) -> Self {
		Self { map: map.into_iter().map(|(k, v)| (ObjectRef::from(k), ObjectRef::from(v))).collect() }
	}
}

impl<'rt> FromIterator<(ObjectRef<'rt>, ObjectRef<'rt>)> for Map<'rt> {
	fn from_iter<T: IntoIterator<Item = (ObjectRef<'rt>, ObjectRef<'rt>)>>(iter: T) -> Self {
		Self { map: iter.into_iter().collect() }
	}
}

impl<'rt> FromIterator<(ObjectRef<'rt>, Object<'rt>)> for Map<'rt> {
	fn from_iter<T: IntoIterator<Item = (ObjectRef<'rt>, Object<'rt>)>>(iter: T) -> Self {
		Self { map: iter.into_iter().map(|(k, v)| (k, ObjectRef::from(v))).collect() }
	}
}

impl<'rt> FromIterator<(Object<'rt>, ObjectRef<'rt>)> for Map<'rt> {
	fn from_iter<T: IntoIterator<Item = (Object<'rt>, ObjectRef<'rt>)>>(iter: T) -> Self {
		Self { map: iter.into_iter().map(|(k, v)| (ObjectRef::from(k), v)).collect() }
	}
}

impl<'rt> FromIterator<(Object<'rt>, Object<'rt>)> for Map<'rt> {
	fn from_iter<T: IntoIterator<Item = (Object<'rt>, Object<'rt>)>>(iter: T) -> Self {
		Self { map: iter.into_iter().map(|(k, v)| (ObjectRef::from(k), ObjectRef::from(v))).collect() }
	}
}
