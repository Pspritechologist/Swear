use super::*;

#[derive(Clone, Default, PartialEq, Eq)]
#[swear_object]
pub struct State {
	pub state: bool,
}

impl State {
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

	fn to_swear_deck(&self) -> Deck {
		Deck::from(vec![Object::from(self.clone())])
	}

	fn to_swear_map(&self) -> Map {
		Map::from(vec![(Object::from(Chars::from("state")), Object::from(self.clone()))])
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
