use super::*;

#[derive(Debug, Clone)]
pub struct Count {
	pub value: f64,
}

impl Count {
	pub fn new(value: f64) -> Self {
		Self { value }
	}
}

impl SwearObject for Count {
	fn to_chars(&self) -> Chars {
		todo!()
	}

	fn to_state(&self) -> State {
		todo!()
	}

	fn to_count(&self) -> Count {
		self.clone()
	}

	fn to_deck(&self) -> Deck {
		todo!()
	}

	fn to_map(&self) -> Map {
		todo!()
	}
}
