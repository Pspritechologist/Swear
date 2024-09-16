use super::*;

#[derive(Clone, Default, PartialEq, Eq)]
pub struct State {
	pub state: bool,
}

impl IObject for State {
	fn to_chars(&self) -> Chars {
		match self.state {
			true => "positive",
			false => "negative",
		}.into()
	}

	fn to_count(&self) -> Count {
		match self.state {
			true => 1,
			false => 0,
		}.into()
	}

	fn to_state(&self) -> State {
		self.clone()
	}

	fn to_deck(&self) -> Deck {
		Deck::from(vec![Object::from(self.clone())])
	}

	fn to_map(&self) -> Map {
		Map::from(vec![(Object::from(Chars::from("state")), Object::from(self.clone()))])
	}

	fn object_name(&self) -> &str {
		"State"
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
