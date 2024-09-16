use super::*;

#[derive(Clone, Default, PartialEq, Eq)]
pub struct Zip;

impl IObject for Zip {
	fn to_chars(&self) -> Chars {
		Chars::default()
	}

	fn to_count(&self) -> Count {
		Count::default()
	}

	fn to_state(&self) -> State {
		State::default()
	}

	fn to_zip(&self) -> Zip {
		self.clone()
	}

	fn to_deck(&self) -> Deck {
		Deck::default()
	}

	fn to_map(&self) -> Map {
		Map::default()
	}

	fn is_zip(&self) -> bool {
		true
	}

	fn object_name(&self) -> &str {
		"Zip"
	}
}

impl std::fmt::Debug for Zip {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "Zip")
	}
}
