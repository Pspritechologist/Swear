use crate::runtime::ObjectRef;

use super::*;

#[cfg_attr(feature="serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Default, PartialEq, Eq)]
#[swear_object]
pub struct Deck<'rt> {
	pub deck: Vec<ObjectRef<'rt>>,
}

impl<'rt> Deck<'rt> {
	pub fn from_vec_lit(v: &Vec<ObjectLiteral>) -> Self {
		Self { deck: v.iter().map(|l| Object::from_literal(l).into()).collect() }
	}

	pub fn from_iter_ref<I: IntoIterator<Item = ObjectRef<'rt>>>(iter: I) -> Self {
		Self { deck: iter.into_iter().collect() }
	}

	pub fn from_iter_obj<I: IntoIterator<Item = Object<'rt>>>(iter: I) -> Self {
		Self { deck: iter.into_iter().map(ObjectRef::from).collect() }
	}

	pub fn from_iter_lit<I: IntoIterator<Item = ObjectLiteral>>(iter: I) -> Self {
		Self { deck: iter.into_iter().map(|l| Object::from_literal(&l).into()).collect() }
	}
}

impl<'rt> Deck<'rt> {
	fn to_swear_chars(&self) -> Chars {
		let mut chars = String::with_capacity(self.deck.len() * 4);
		for s in self.deck.iter().map(|o| o.access().to_chars().chars) {
			chars.push_str(&s);
			chars.push(' ');
		}
		chars.pop();
		chars.into()
	}

	fn to_swear_count(&self) -> Count {
		self.deck.len().into()
	}

	fn to_swear_state(&self) -> State {
		(!self.deck.is_empty()).into()
	}

	fn to_swear_deck(&self) -> Deck<'rt> {
		self.clone()
	}

	fn to_swear_map(&self) -> Map<'rt> {
		Map::from(self.deck
			.iter()
			.enumerate()
			.map(|(i, o)| (Count::from(i).into(), o.clone()))
			.collect::<Vec<(Object, _)>>())
	}

	fn get_functions(&self) -> HashMap<String, FunctionInfo<'rt>> {
		let mut functions = HashMap::new();

		// Lest function.
		// No op, returns this Object.
		functions.insert("lest".to_string(), FunctionInfoBuilder::new("lest".to_string()).build_native(Arc::new(Mutex::new(|obj, _| Ok(Some(obj)) ))));

		// Solid function.
		// Returns false if Zip.
		functions.insert("solid".to_string(), FunctionInfoBuilder::new("solid".to_string()).build_native(Arc::new(Mutex::new(|_, _| Ok(Some(Object::from(State::from(true)).into())) ))));

		functions
	}
}

impl<'rt> std::fmt::Debug for Deck<'rt> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let mut debug = f.debug_list();
		for o in self.deck.iter() {
			debug.entry(&o.access());
		}
		debug.finish()
	}
}

impl<'rt> From<Vec<ObjectRef<'rt>>> for Deck<'rt> {
	fn from(deck: Vec<ObjectRef<'rt>>) -> Self {
		Self { deck }
	}
}

impl<'rt> From<Vec<Object<'rt>>> for Deck<'rt> {
	fn from(deck: Vec<Object<'rt>>) -> Self {
		Self { deck: deck.into_iter().map(ObjectRef::from).collect() }
	}
}
