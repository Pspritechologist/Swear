use std::{env::current_exe, fmt::Debug};

use crate::dyn_libraries;

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
		Count { count: self.chars.parse().unwrap_or_else(|_| crate::BigNum::from(self.chars.len())) }
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

	fn get_functions(&self) -> HashMap<String, FunctionInfo<'rt>> {
		let mut functions = HashMap::new();

		// Scribe function.
		// Prints the characters to the console.
		functions.insert("scribe".to_string(), FunctionInfoBuilder::new("scribe".to_string()).build(Arc::new(Mutex::new(|obj: ObjectRef<'rt>, _| {
			let lock = obj.access();
			println!("{}", lock.to_chars().chars);
			Ok(None)
		}))));

		// Concat function.
		// Takes any number of arguments and concatenates them into a single string separated by the method target.
		functions.insert("concat".to_string(), FunctionInfoBuilder::new("concat".to_string()).build(Arc::new(Mutex::new(|obj: ObjectRef<'rt>, args: Vec<ObjectRef<'rt>>| {
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
		}))));

		// Size function.
		// Returns the number of characters in the string.
		functions.insert("size".to_string(), FunctionInfoBuilder::new("size".to_string()).build(Arc::new(Mutex::new(|obj: ObjectRef<'rt>, _| {
			let lock = obj.access();
			Ok(Some(Object::from(Count::from(crate::BigNum::from(lock.to_chars().chars.len()))).into()))
		}))));

		// functions.insert("load".to_string(), FunctionInfoBuilder::new("load".to_string()).build(Arc::new(Mutex::new(|obj: ObjectRef<'rt>, _| {
		// 	//TODO: Customize lib loading.
		// 	let lock = obj.access();
		// 	dyn_libraries::load_library(
		// 		// &current_exe().unwrap().with_file_name(&lock.to_chars().chars).with_extension("slur") //? Non-debug.
		// 		&current_exe().unwrap().with_file_name(format!("libswear_{}", &lock.to_chars().chars)).with_extension("so")
		// 	).map(|o| Some(o)).map_err(|_| eprintln!("Failed to load library!"))
		// }))));

		functions
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
