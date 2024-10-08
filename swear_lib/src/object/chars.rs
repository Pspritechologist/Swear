use crate::dyn_libraries;
use std::{env::current_exe, fmt::Debug};

use super::*;

/// An object that represents a collection of zero or more unicode characters.
#[cfg_attr(feature="serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Default, PartialEq, Eq)]
#[swear_object]
pub struct Chars {
	pub chars: String,
}

impl<'rt> Chars {
	fn to_swear_chars(&self) -> Chars {
		self.clone()
	}

	fn to_swear_count(&self) -> Count {
		Count { count: self.chars.parse().unwrap_or_else(|_| self.chars.len() as i64) }
	}

	fn to_swear_state(&self) -> State {
		State { state: !self.chars.is_empty() }
	}

	fn to_swear_deck(&self) -> Deck<'rt> {
		Deck { deck: self.chars.split_whitespace().map(|s| Object::from(Chars::from(s)).into()).collect() }
	}

	fn to_swear_map(&self) -> Map<'rt> {
		Map { map: self.chars.split_whitespace().map(|s| (Object::from(Chars::from(s)).into(), Object::from(Zip).into())).collect() }
	}

	fn get_function(&self, name: &str) -> Option<FunctionInfo<'rt>> {
		Some(match name {
			// Scribe function.
			// Prints the characters to the console.
			"scribe" =>
				FunctionInfoBuilder::new("scribe".to_string()).build_native(Arc::new(Mutex::new(|obj: ObjectRef<'rt>, _| {
					let lock = obj.access();
					println!("{}", lock.to_chars().chars);
					Ok(None)
				}))),
		
			// Concat function.
			// Takes any number of arguments and concatenates them into a single string separated by the method target.
			"concat" =>
				FunctionInfoBuilder::new("concat".to_string()).build_native(Arc::new(Mutex::new(|obj: ObjectRef<'rt>, args: Vec<ObjectRef<'rt>>| {
					let target = obj.access();
					let mut result = String::new();
					let mut iter = args.iter();
					while let Some(obj) = iter.next() {
						let lock = obj.access();
						result.push_str(&lock.to_chars().chars);
						if iter.len() > 0 {
							result.push_str(&target.to_chars().chars);
						}
					}

					Ok(Some(Object::from(Chars::from(result)).into()))
				}))),
		
			// Size function.
			// Returns the number of characters in the string.
			"size" =>
				FunctionInfoBuilder::new("size".to_string()).build_native(Arc::new(Mutex::new(|obj: ObjectRef<'rt>, _| {
					let lock = obj.access();
					Ok(Some(Object::from(Count::from(lock.to_chars().chars.len())).into()))
				}))),
		
			// Assign function.
			// Replaces the value of the Chars in place.
			"assign" =>
				FunctionInfoBuilder::new("assign".to_string()).build_native(Arc::new(Mutex::new(|obj: ObjectRef<'rt>, args: Vec<ObjectRef<'rt>>| {
					let mut lock = obj.lock();
					let new_value = args.get(0).ok_or(())?.access();
					lock.as_chars_mut().unwrap().chars = new_value.to_chars().chars;
					drop(lock);
					Ok(Some(obj))
				}))),
		
			// Lest function.
			// No op, returns this Object.
			"lest" =>
				FunctionInfoBuilder::new("lest".to_string()).build_native(Arc::new(Mutex::new(|obj, _| Ok(Some(obj))))),
		
			// Solid function.
			// Returns false if Zip.
			"solid" =>
				FunctionInfoBuilder::new("solid".to_string()).build_native(Arc::new(Mutex::new(|_, _| Ok(Some(Object::from(State::from(true)).into()))))),

			_ => return None,
		})
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
