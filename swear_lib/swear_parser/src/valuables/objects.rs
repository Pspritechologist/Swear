use super::*;

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
		ObjectLiteral::Count(escape_text(value).parse().unwrap_or(1.))
	}

	pub fn new_zip(_value: &str) -> Self {
		ObjectLiteral::Zip
	}

	pub fn new_deck(value: &str) -> Self {
		ObjectLiteral::Deck(escape_text(value)
			.split_ascii_whitespace()
			.map(ObjectLiteral::new_chars)
			.collect())
	}

	pub fn new_map(value: &str) -> Self {
		ObjectLiteral::Map(escape_text(value)
			.split_ascii_whitespace()
			.map(|s| (ObjectLiteral::new_chars(&s), ObjectLiteral::Zip))
			.collect())
	}
}

fn escape_text(text: &str) -> String {
	if text.len() < 2 {
		return "".into();
	}
	text.replace("~", "~~")[1..text.len() - 1].replace("~'", "'")
}

#[derive(Debug, Clone)]
pub enum ObjectConversion {
	ToChars(Valuable),
	ToState(Valuable),
	ToCount(Valuable),
	ToZip(Valuable),
	ToDeck(Valuable),
	ToMap(Valuable),
}

impl ObjectConversion {
	pub fn new_to_chars(value: Valuable) -> Self {
		ObjectConversion::ToChars(value)
	}

	pub fn new_to_state(value: Valuable) -> Self {
		ObjectConversion::ToState(value)
	}

	pub fn new_to_count(value: Valuable) -> Self {
		ObjectConversion::ToCount(value)
	}

	pub fn new_to_zip(value: Valuable) -> Self {
		ObjectConversion::ToZip(value)
	}

	pub fn new_to_deck(value: Valuable) -> Self {
		ObjectConversion::ToDeck(value)
	}

	pub fn new_to_map(value: Valuable) -> Self {
		ObjectConversion::ToMap(value)
	}
}
