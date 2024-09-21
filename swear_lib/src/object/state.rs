use super::*;

#[cfg_attr(feature="serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Default, PartialEq, Eq)]
#[swear_object]
pub struct State {
	pub state: bool,
}

impl<'rt> State {
	fn to_swear_chars(&self) -> Chars {
		match self.state {
			true => "positive",
			false => "negative",
		}.into()
	}

	fn to_swear_count(&self) -> Count {
		match self.state {
			true => 1,
			false => 0,
		}.into()
	}

	fn to_swear_state(&self) -> State {
		self.clone()
	}

	fn to_swear_deck(&self) -> Deck<'rt> {
		Deck::from(vec![Object::from(self.clone())]) //TODO: Shouldn't these be ObjectRefs?
	}

	fn to_swear_map(&self) -> Map<'rt> {
		Map::from(vec![(Object::from(Chars::from("state")), Object::from(self.clone()))]) //TODO: Shouldn't these be ObjectRefs?
	}
}

impl std::fmt::Debug for State {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "State({:?})", self.state)
	}
}

impl From<bool> for State {
	fn from(state: bool) -> Self {
		Self { state }
	}
}
