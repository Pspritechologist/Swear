use std::ops::{Deref, DerefMut};

use super::*;

#[cfg_attr(feature="serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Default)]
pub struct Expression {
	pub contents: Vec<TopLevelItem>,
}

impl Expression {
	pub fn new(contents: Vec<TopLevelItem>) -> Self {
		Expression { contents }
	}
}

impl Deref for Expression {
	type Target = Vec<TopLevelItem>;

	fn deref(&self) -> &Self::Target {
		&self.contents
	}
}

impl DerefMut for Expression {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.contents
	}
}

impl From<Vec<TopLevelItem>> for Expression {
	fn from(contents: Vec<TopLevelItem>) -> Self {
		Expression::new(contents)
	}
}

impl From<Expression> for Vec<TopLevelItem> {
	fn from(expr: Expression) -> Self {
		expr.contents
	}
}
