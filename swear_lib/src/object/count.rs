use super::*;

#[cfg_attr(feature="serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Default, PartialEq, Eq)]
#[swear_object]
pub struct Count {
	pub count: i64,
}

impl<'rt> Count {
	fn to_swear_chars(&self) -> Chars {
		Chars { chars: self.count.to_string() }
	}

	fn to_swear_count(&self) -> Count {
		self.clone()
	}

	fn to_swear_state(&self) -> State {
		State { state: self.count > 1 }
	}

	fn to_swear_deck(&self) -> Deck<'rt> {
		let mut i = self.count.clone();
		let mut deck = vec![];
		while i > 0 {
			deck.push(Object::from(Count::from(i.clone())).into());
			i -= 1;
		}

		Deck { deck }
	}

	fn to_swear_map(&self) -> Map<'rt> {
		Map::default()
	}

	fn get_function(&self, name: &str) -> Option<FunctionInfo<'rt>> {
		Some(match name {
			
		// Add function.
		// Adds all arguments to the count.
		"add" => FunctionInfoBuilder::new("add".to_string()).build_native(Arc::new(Mutex::new(|obj: ObjectRef<'rt>, args: Vec<ObjectRef<'rt>>| {
			let mut count_lock = obj.lock();
			let count = count_lock.as_count_mut().unwrap();

			let mut args = args.iter();
			while let Some(arg) = args.next() {
				let arg = arg.access();
				count.count += arg.to_count().count;
			}

			drop(count_lock);

			Ok(Some(obj))
		}))),

		// Sub function.
		// Subtracts all arguments from the count.
		"sub" => FunctionInfoBuilder::new("sub".to_string()).build_native(Arc::new(Mutex::new(|obj: ObjectRef<'rt>, args: Vec<ObjectRef<'rt>>| {
			let mut count_lock = obj.lock();
			let count = count_lock.as_count_mut().unwrap();

			let mut args = args.iter();
			while let Some(arg) = args.next() {
				let arg = arg.access();
				count.count -= arg.to_count().count;
			}

			drop(count_lock);

			Ok(Some(obj))
		}))),

		// Mul function.
		// Multiplies by all arguments one after the other.
		"mul" => FunctionInfoBuilder::new("mul".to_string()).build_native(Arc::new(Mutex::new(|obj: ObjectRef<'rt>, args: Vec<ObjectRef<'rt>>| {
			let mut count_lock = obj.lock();
			let count = count_lock.as_count_mut().unwrap();

			for arg in args {
				let arg = arg.access();
				count.count *= arg.to_count().count;
			}

			drop(count_lock);

			Ok(Some(obj))
		}))),

		// Div function.
		// Multiplies by all arguments one after the other.
		"div" => FunctionInfoBuilder::new("div".to_string()).build_native(Arc::new(Mutex::new(|obj: ObjectRef<'rt>, args: Vec<ObjectRef<'rt>>| {
			let mut count_lock = obj.lock();
			let count = count_lock.as_count_mut().unwrap();

			for arg in args {
				let arg = arg.access();
				count.count /= arg.to_count().count;
			}

			drop(count_lock);

			Ok(Some(obj))
		}))),

		// Equals function.
		// Returns true if all arguments are equal to the count.
		"equals" => FunctionInfoBuilder::new("equals".to_string()).build_native(Arc::new(Mutex::new(|obj: ObjectRef<'rt>, args: Vec<ObjectRef<'rt>>| {
			let count_lock = obj.access();
			let count = count_lock.as_count().unwrap();
		
			for arg in args {
				let arg = arg.access();
				if count.count != arg.to_count().count {
					return Ok(Some(Object::from(State::from(false)).into()));
				}
			}

			Ok(Some(Object::from(State::from(true)).into()))
		}))),

		// Greater function.
		// Returns true if all arguments are less than the count.
		"greater" => FunctionInfoBuilder::new("greater".to_string()).build_native(Arc::new(Mutex::new(|obj: ObjectRef<'rt>, args: Vec<ObjectRef<'rt>>| {
			let count_lock = obj.access();
			let count = count_lock.as_count().unwrap();

			for arg in args {
				let arg = arg.access();
				if count.count <= arg.to_count().count {
					return Ok(Some(Object::from(State::from(false)).into()));
				}
			}

			Ok(Some(Object::from(State::from(true)).into()))
		}))),

		// Less function.
		// Returns true if all arguments are greater than the count.
		"less" => FunctionInfoBuilder::new("less".to_string()).build_native(Arc::new(Mutex::new(|obj: ObjectRef<'rt>, args: Vec<ObjectRef<'rt>>| {
			let count_lock = obj.access();
			let count = count_lock.as_count().unwrap();

			for arg in args {
				let arg = arg.access();
				if count.count >= arg.to_count().count {
					return Ok(Some(Object::from(State::from(false)).into()));
				}
			}

			Ok(Some(Object::from(State::from(true)).into()))
		}))),

		// Greateq function.
		// Returns true if all arguments are less than or equal to the count.
		"greateq" => FunctionInfoBuilder::new("greateq".to_string()).build_native(Arc::new(Mutex::new(|obj: ObjectRef<'rt>, args: Vec<ObjectRef<'rt>>| {
			let count_lock = obj.access();
			let count = count_lock.as_count().unwrap();

			for arg in args {
				let arg = arg.access();
				if count.count < arg.to_count().count {
					return Ok(Some(Object::from(State::from(false)).into()));
				}
			}

			Ok(Some(Object::from(State::from(true)).into()))
		}))),

		// Lesseq function.
		// Returns true if all arguments are greater than or equal to the count.
		"lesseq" => FunctionInfoBuilder::new("lesseq".to_string()).build_native(Arc::new(Mutex::new(|obj: ObjectRef<'rt>, args: Vec<ObjectRef<'rt>>| {
			let count_lock = obj.access();
			let count = count_lock.as_count().unwrap();

			for arg in args {
				let arg = arg.access();
				if count.count > arg.to_count().count {
					return Ok(Some(Object::from(State::from(false)).into()));
				}
			}

			Ok(Some(Object::from(State::from(true)).into()))
		}))),

		// Round function.
		// Rounds the count to the nearest whole number.
		"round" => FunctionInfoBuilder::new("round".to_string()).build_native(Arc::new(Mutex::new(|obj: ObjectRef<'rt>, _| {
			let mut count_lock = obj.lock();
			let count = count_lock.as_count_mut().unwrap();

			// count.count = count.count.round();

			drop(count_lock);

			Ok(Some(obj))
		}))),

		// Lest function.
		// No op, returns this Object.
		"lest" => FunctionInfoBuilder::new("lest".to_string()).build_native(Arc::new(Mutex::new(|obj, _| Ok(Some(obj))))),

		// Solid function.
		// Returns false if Zip.
		"solid" => FunctionInfoBuilder::new("solid".to_string()).build_native(Arc::new(Mutex::new(|_, _| Ok(Some(Object::from(State::from(true)).into()))))),

			_ => return None,
		})
	}
}

impl std::fmt::Debug for Count {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "Count({:?})", self.count)
	}
}

impl From<i64> for Count {
	fn from(count: i64) -> Self {
		Self { count }
	}
}

// impl From<f64> for Count {
// 	fn from(count: f64) -> Self {
// 		Self { count: count.into() }
// 	}
// }

// impl From<f32> for Count {
// 	fn from(count: f32) -> Self {
// 		Self { count: count.into() }
// 	}
// }

// impl From<i64> for Count {
// 	fn from(count: i64) -> Self {
// 		Self { count: count.into() }
// 	}
// }

impl From<u64> for Count {
	fn from(count: u64) -> Self {
		Self { count: count as i64 }
	}
}

impl From<i32> for Count {
	fn from(count: i32) -> Self {
		Self { count: count.into() }
	}
}

impl From<u32> for Count {
	fn from(count: u32) -> Self {
		Self { count: count.into() }
	}
}

impl From<isize> for Count {
	fn from(count: isize) -> Self {
		Self { count: count as i64 }
	}
}

impl From<usize> for Count {
	fn from(count: usize) -> Self {
		Self { count: count as i64 }
	}
}
