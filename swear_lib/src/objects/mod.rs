mod chars;
mod state;
mod count;
mod zip;
mod deck;
mod map;
mod dynamic;

pub use chars::Chars;
pub use state::State;
pub use count::Count;
pub use zip::Zip;
pub use deck::Deck;
pub use map::Map;
pub use dynamic::Dynamic;

use enum_dispatch::enum_dispatch;

use super::*;

#[enum_dispatch(SwearObject)]
#[derive(Debug, Clone)]
pub enum Object {
	Chars,
	State,
	Count,
	Zip,
	Deck,
	Map,
	Dynamic,
}

impl Default for Object {
	fn default() -> Self {
		Object::Zip(Zip::new())
	}
}

#[enum_dispatch]
pub trait SwearObject {
	fn to_chars(&self) -> Chars;
	fn to_state(&self) -> State;
	fn to_count(&self) -> Count;
	fn to_zip(&self) -> Zip {
		Zip::new()
	}
	fn to_deck(&self) -> Deck;
	fn to_map(&self) -> Map;

	fn get_methods(&self) -> Vec<&'static str> {
		vec![]
	}
}
