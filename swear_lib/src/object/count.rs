use super::*;

#[cfg_attr(feature="serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Default, PartialEq, Eq)]
#[swear_object]
pub struct Count {
	pub count: crate::BigNum,
}

impl<'rt> Count {
	fn to_swear_chars(&self) -> Chars {
		Chars { chars: self.count.to_decimal().value().to_string() }
	}

	fn to_swear_count(&self) -> Count {
		self.clone()
	}

	fn to_swear_state(&self) -> State {
		State { state: self.count > crate::BigNum::ONE }
	}

	fn to_swear_deck(&self) -> Deck<'rt> {
		let mut i = self.count.clone();
		let mut deck = vec![];
		while i.gt(&crate::BigNum::ZERO) {
			deck.push(Object::from(Count::from(i.clone())).into());
			i -= crate::BigNum::ONE;
		}

		Deck { deck }
	}

	fn to_swear_map(&self) -> Map<'rt> {
		Map::default()
	}

	fn get_functions(&self) -> HashMap<String, FunctionInfo<'rt>> {
		let mut functions = HashMap::new();

		// Arithmetic.

		// Add function.
		// Adds all arguments to the count.
		functions.insert("add".to_string(), FunctionInfoBuilder::new("add".to_string()).build(Arc::new(Mutex::new(|obj: ObjectRef<'rt>, args: Vec<ObjectRef<'rt>>| {
			let mut count_lock = obj.lock();
			let count = count_lock.as_count_mut().unwrap();

			let mut args = args.iter();
			while let Some(arg) = args.next() {
				let arg = arg.access();
				count.count += arg.to_count().count;
			}

			drop(count_lock);

			Ok(Some(obj))
		}))));

		// Sub function.
		// Subtracts all arguments from the count.
		functions.insert("sub".to_string(), FunctionInfoBuilder::new("sub".to_string()).build(Arc::new(Mutex::new(|obj: ObjectRef<'rt>, args: Vec<ObjectRef<'rt>>| {
			let mut count_lock = obj.lock();
			let count = count_lock.as_count_mut().unwrap();

			let mut args = args.iter();
			while let Some(arg) = args.next() {
				let arg = arg.access();
				count.count -= arg.to_count().count;
			}

			drop(count_lock);

			Ok(Some(obj))
		}))));

		functions
	}
}

impl std::fmt::Debug for Count {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "Count({:?})", self.count)
	}
}

impl From<crate::BigNum> for Count {
	fn from(count: crate::BigNum) -> Self {
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

impl From<i64> for Count {
	fn from(count: i64) -> Self {
		Self { count: count.into() }
	}
}

impl From<u64> for Count {
	fn from(count: u64) -> Self {
		Self { count: count.into() }
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
		Self { count: count.into() }
	}
}

impl From<usize> for Count {
	fn from(count: usize) -> Self {
		Self { count: count.into() }
	}
}
