use crate::runtime::ObjectRef;

use super::*;

#[cfg_attr(feature="serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Default, PartialEq, Eq)]
#[swear_object]
pub struct Deck {
	pub deck: Vec<ObjectRef>,
}

impl Deck {
	pub fn from_iter_ref<I: IntoIterator<Item = ObjectRef>>(iter: I) -> Self {
		Self { deck: iter.into_iter().collect() }
	}

	pub fn from_iter_obj<I: IntoIterator<Item = Object>>(iter: I) -> Self {
		Self { deck: iter.into_iter().map(ObjectRef::from).collect() }
	}

	pub fn from_iter_lit<I: IntoIterator<Item = ObjectLiteral>>(iter: I) -> Self {
		Self { deck: iter.into_iter().map(|l| Object::from(l).into()).collect() }
	}
}

impl Deck {
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

	fn to_swear_deck(&self) -> Deck {
		self.clone()
	}

	fn to_swear_map(&self) -> Map {
		Map::from(self.deck
			.iter()
			.enumerate()
			.map(|(i, o)| (Count::from(i).into(), o.clone()))
			.collect::<Vec<(Object, _)>>())
	}
}

impl std::fmt::Debug for Deck {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let mut debug = f.debug_list();
		for o in self.deck.iter() {
			debug.entry(&o.access());
		}
		debug.finish()
	}
}

impl From<Vec<ObjectRef>> for Deck {
	fn from(deck: Vec<ObjectRef>) -> Self {
		Self { deck }
	}
}

impl From<Vec<Object>> for Deck {
	fn from(deck: Vec<Object>) -> Self {
		Self { deck: deck.into_iter().map(ObjectRef::from).collect() }
	}
}
