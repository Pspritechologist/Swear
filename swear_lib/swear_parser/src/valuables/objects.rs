use super::*;

#[cfg_attr(feature="serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone)]
pub enum ObjectLiteral {
	Chars(String),
	State(bool),
	Count(f64),
	Zip,
	Deck(Vec<ObjectLiteral>),
	Map(Vec<(ObjectLiteral, ObjectLiteral)>),
}

impl ObjectLiteral {
	pub fn new_chars(value: &str) -> Self {
		ObjectLiteral::Chars(escape_text(value))
	}

	pub fn new_state(_value: &str) -> Self {
		ObjectLiteral::State(true)
	}

	pub fn new_count(value: &str) -> Self {
		ObjectLiteral::Count(escape_text(value).parse().unwrap_or(1.)) //FIXME
	}

	pub fn new_zip(_value: &str) -> Self {
		ObjectLiteral::Zip
	}

	pub fn new_deck(value: &str) -> Self {
		ObjectLiteral::Deck(escape_text(value)
			.split_ascii_whitespace()
			.map(|s| ObjectLiteral::Chars(s.into()))
			.collect())
	}

	pub fn new_map(value: &str) -> Self {
		ObjectLiteral::Map(escape_text(value)
			.split_ascii_whitespace()
			.map(|s| (ObjectLiteral::Chars(s.into()), ObjectLiteral::Zip))
			.collect())
	}
}

//TODO: Escaping probably shouldn't happen here.
fn escape_text(text: &str) -> String {
	if text.len() < 2 {
		return "".into();
	}
	text.replace("~", "~~")[1..text.len() - 1].replace("~'", "'")
}

#[cfg_attr(feature="serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy)]
pub enum ObjectSymbol {
	Chars,
	State,
	Count,
	Zip,
	Deck,
	Map,
}

#[cfg_attr(feature="serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone)]
pub struct ObjectConversion {
	pub value: Valuable,
	pub symbol: ObjectSymbol,
	// ToChars(Valuable),
	// ToState(Valuable),
	// ToCount(Valuable),
	// ToZip(Valuable),
	// ToDeck(Valuable),
	// ToMap(Valuable),
}

impl ObjectConversion {
	pub fn new_to_chars(value: Valuable) -> Self {
		ObjectConversion {
			value,
			symbol: ObjectSymbol::Chars,
		}
	}

	pub fn new_to_state(value: Valuable) -> Self {
		ObjectConversion {
			value,
			symbol: ObjectSymbol::State,
		}
	}

	pub fn new_to_count(value: Valuable) -> Self {
		ObjectConversion {
			value,
			symbol: ObjectSymbol::Count,
		}
	}

	pub fn new_to_zip(value: Valuable) -> Self {
		ObjectConversion {
			value,
			symbol: ObjectSymbol::Zip,
		}
	}

	pub fn new_to_deck(value: Valuable) -> Self {
		ObjectConversion {
			value,
			symbol: ObjectSymbol::Deck,
		}
	}

	pub fn new_to_map(value: Valuable) -> Self {
		ObjectConversion {
			value,
			symbol: ObjectSymbol::Map,
		}
	}
}
