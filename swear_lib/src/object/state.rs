use super::*;

#[cfg_attr(feature="serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Default, PartialEq, Eq)]
#[swear_object]
pub struct State {
	pub state: bool,
}

impl<'rt> State {
	fn to_swear_chars(&self) -> Chars {
		match self.state {
			true => "positive",
			false => "negative",
		}.into()
	}

	fn to_swear_count(&self) -> Count {
		match self.state {
			true => 1,
			false => 0,
		}.into()
	}

	fn to_swear_state(&self) -> State {
		self.clone()
	}

	fn to_swear_deck(&self) -> Deck<'rt> {
		Deck::from(vec![Object::from(self.clone())]) //TODO: Shouldn't these be ObjectRefs?
	}

	fn to_swear_map(&self) -> Map<'rt> {
		Map::from(vec![(Object::from(Chars::from("state")), Object::from(self.clone()))]) //TODO: Shouldn't these be ObjectRefs?
	}

	fn get_functions(&self) -> HashMap<String, FunctionInfo<'rt>> {
		let mut functions = HashMap::new();

		// And function.
		// Returns true if this object and all arguments are true.
		functions.insert("and".to_string(), FunctionInfoBuilder::new("and".to_string()).build_native(Arc::new(Mutex::new(|obj: ObjectRef<'rt>, args: Vec<ObjectRef<'rt>>| {
			let state_lock = obj.access();
			let state = state_lock.as_state().unwrap();

			if !state.state {
				return Ok(Some(Object::from(State::from(false)).into()));
			}

			let mut args = args.iter();
			while let Some(arg) = args.next() {
				let arg = arg.access();
				let arg = arg.to_state();
				if !arg.state {
					return Ok(Some(Object::from(State::from(false)).into()));
				}
			}
			
			Ok(Some(Object::from(State::from(true)).into()))
		}))));

		// Lest function.
		// No op, returns this Object.
		functions.insert("lest".to_string(), FunctionInfoBuilder::new("lest".to_string()).build_native(Arc::new(Mutex::new(|obj, _| Ok(Some(obj)) ))));

		// Solid function.
		// Returns false if Zip.
		functions.insert("solid".to_string(), FunctionInfoBuilder::new("solid".to_string()).build_native(Arc::new(Mutex::new(|_, _| Ok(Some(Object::from(State::from(true)).into())) ))));

		functions
	}
}

impl std::fmt::Debug for State {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "State({:?})", self.state)
	}
}

impl From<bool> for State {
	fn from(state: bool) -> Self {
		Self { state }
	}
}
