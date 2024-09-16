use super::*;

#[cfg_attr(feature="serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone)]
pub struct Expression {
	pub contents: Vec<TopLevelItem>,
}

impl Expression {
	pub fn new(contents: Vec<TopLevelItem>) -> Self {
		Expression { contents }
	}
}
