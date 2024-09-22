use super::*;

#[cfg_attr(feature="serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Default, PartialEq, Eq)]
// #[swear_object]
pub struct Zip;

impl<'rt> IObject<'rt> for Zip {
	fn to_chars(&self) -> Chars {
		Chars::default()
	}

	fn to_count(&self) -> Count {
		Count::default()
	}

	fn to_state(&self) -> State {
		State::default()
	}

	fn to_deck(&self) -> Deck<'rt> {
		Deck::default()
	}

	fn to_map(&self) -> Map<'rt> {
		Map::default()
	}

	fn is_zip(&self) -> bool {
		true
	}

	fn get_info(&self) -> ObjectInfo {
		ObjectInfo::from_str("Zip")
			.with_description_str("An Object that represents nothing. Nada. Zilch.")
	}

	fn get_functions(&self) -> HashMap<String, FunctionInfo<'rt>> {
		let mut functions = HashMap::new();

		// Lest function.
		// Returns the first argument.
		functions.insert("lest".to_string(), FunctionInfoBuilder::new("lest".to_string()).build_native(Arc::new(Mutex::new(|_, args: Vec<ObjectRef<'rt>>|
			match args.first() {
				Some(arg) => Ok(Some(arg.copy())),
				None => Err(()),
			}
		))));

		// Solid function.
		// Returns false if Zip.
		functions.insert("solid".to_string(), FunctionInfoBuilder::new("solid".to_string()).build_native(Arc::new(Mutex::new(|_, _| Ok(Some(Object::from(State::from(false)).into())) ))));

		functions
	}
}

impl std::fmt::Debug for Zip {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "Zip")
	}
}
