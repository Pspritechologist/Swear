use super::*;

#[derive(Debug, Clone)]
pub struct Deck {
	pub value: Vec<Object>,
}

impl Deck {
	pub fn new(value: Vec<Object>) -> Self {
		Self { value }
	}
}

impl SwearObject for Deck {
	fn to_chars(&self) -> Chars {
		todo!()
	}

	fn to_state(&self) -> State {
		todo!()
	}

	fn to_count(&self) -> Count {
		todo!()
	}

	fn to_deck(&self) -> Deck {
		self.clone()
	}

	fn to_map(&self) -> Map {
		todo!()
	}
}
