use std::fmt::Debug;

use super::*;

#[derive(Clone, Default, PartialEq, Eq)]
pub struct Chars {
	pub chars: String,
}

impl IObject for Chars {
	fn to_chars(&self) -> Chars {
		self.clone()
	}

	fn to_count(&self) -> Count {
		Count { count: self.chars.parse().unwrap_or_else(|_| crate::BigNum::from(self.chars.len())) }
	}

	fn to_state(&self) -> State {
		State { state: !self.chars.is_empty() }
	}

	fn to_deck(&self) -> Deck {
		Deck { deck: self.chars.split_whitespace().map(|s| Object::from(Chars::from(s)).into()).collect() }
	}

	fn to_map(&self) -> Map {
		Map { map: self.chars.split_whitespace().map(|s| (Object::from(Chars::from(s)).into(), Object::from(Zip).into())).collect() }
	}

	fn object_name(&self) ->  &str {
		"Chars"
	}
}

impl Debug for Chars {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "Chars {:?}", self.chars)
	}
}

impl From<String> for Chars {
	fn from(chars: String) -> Self {
		Self { chars }
	}
}

impl From<&str> for Chars {
	fn from(chars: &str) -> Self {
		Self { chars: chars.into() }
	}
}
