use std::{ops::Deref, sync::LazyLock};

use super::*;

static DATA: LazyLock<Mutex<Vec<f64>>> = LazyLock::new(|| Mutex::new(Vec::new()));

#[derive(Clone, Default, PartialEq, Eq)]
// #[swear_object]
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

	fn to_deck(&self) -> Deck {
		Deck::default()
	}

	fn to_map(&self) -> Map {
		Map::default()
	}

	fn is_zip(&self) -> bool {
		true
	}

	fn get_info(&self) -> ObjectInfo {
		ObjectInfo::from_str("Zip")
			.with_description_str("An Object that represents nothing. Nada. Zilch.")
	}
}

impl std::fmt::Debug for Zip {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "Zip")
	}
}
