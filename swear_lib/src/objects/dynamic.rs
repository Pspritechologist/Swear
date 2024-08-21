use super::*;

use dyn_clone::DynClone;

/// A dynamic object that can be used in Swear.
pub trait DynamicObject: DynClone + SwearObject + Debug {}

dyn_clone::clone_trait_object!(DynamicObject);

pub type Dynamic = Box<dyn DynamicObject>;
impl SwearObject for Dynamic {
	fn to_chars(&self) -> Chars {
		(**self).to_chars()
	}
	fn to_state(&self) -> State {
		(**self).to_state()
	}
	fn to_count(&self) -> Count {
		(**self).to_count()
	}
	fn to_zip(&self) -> Zip {
		(**self).to_zip()
	}
	fn to_deck(&self) -> Deck {
		(**self).to_deck()
	}
	fn to_map(&self) -> Map {
		(**self).to_map()
	}
}
