use super::*;

#[derive(Debug, Clone)]
pub struct Map {
	pub value: Vec<(Object, Object)>,
}

impl Map {
	pub fn new(value: Vec<(Object, Object)>) -> Self {
		Self { value }
	}
}

impl SwearObject for Map {
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
		todo!()
	}

	fn to_map(&self) -> Map {
		self.clone()
	}
}
