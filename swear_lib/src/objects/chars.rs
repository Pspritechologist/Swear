use super::*;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Chars {
	pub value: String,
}

impl Chars {
	pub fn new(value: String) -> Self {
		Self { value }
	}
}

impl SwearObject for Chars {
	fn to_chars(&self) -> Chars {
		self.clone()
	}

	fn to_state(&self) -> State {
		todo!()
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
