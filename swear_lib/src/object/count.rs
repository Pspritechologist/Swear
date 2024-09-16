use super::*;

#[derive(Clone, Default, PartialEq, Eq)]
pub struct Count {
	pub count: crate::BigNum,
}

impl IObject for Count {
	fn to_chars(&self) -> Chars {
		Chars { chars: self.count.to_string() }
	}

	fn to_count(&self) -> Count {
		self.clone()
	}

	fn to_state(&self) -> State {
		State { state: self.count > crate::BigNum::ONE }
	}

	fn to_deck(&self) -> Deck {
		let mut i = self.count.clone();
		let mut deck = vec![];
		while i != crate::BigNum::ZERO {
			deck.push(Object::from(Count::from(i.clone())).into());
			i -= crate::BigNum::ONE;
		}

		Deck { deck }
	}

	fn to_map(&self) -> Map {
		Map::default()
	}

	fn object_name(&self) -> &str {
		"Count"
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
