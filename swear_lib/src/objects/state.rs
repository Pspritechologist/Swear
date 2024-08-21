use super::*;

#[derive(Debug, Clone)]
pub struct State {
	pub value: bool,
}

impl State {
	pub fn new(value: bool) -> Self {
		Self { value }
	}
}

impl SwearObject for State {
	fn to_chars(&self) -> Chars {
		todo!()
	}

	fn to_state(&self) -> State {
		self.clone()
	}

	fn to_count(&self) -> Count {
		todo!()
	}

	fn to_deck(&self) -> Deck {
		todo!()
	}

	fn to_map(&self) -> Map {
		todo!()
	}
}
