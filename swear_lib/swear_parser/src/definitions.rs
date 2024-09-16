use super::*;

#[cfg_attr(feature="serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone)]
pub enum Definition {
	Blueprint {
		name: String,
		exprs: Expression,
	},
	Callback {
		name: String,
		parameters: Vec<String>,
		exprs: Expression,
	},
	Register {
		name: String,
		value: Valuable,
	},
}

impl Definition {
	pub fn new_blueprint(name: &str, contents: Expression) -> Self {
		Definition::Blueprint {
			name: name.into(),
			exprs: contents,
		}
	}

	pub fn new_callback(name: &str, parameters: Vec<&str>, contents: Expression) -> Self {
		Definition::Callback {
			name: name.into(),
			parameters: parameters.iter().map(|s| s.to_string()).collect(),
			exprs: contents,
		}
	}

	pub fn new_register(name: &str, value: Valuable) -> Self {
		Definition::Register {
			name: name.into(),
			value,
		}
	}
}
