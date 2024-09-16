mod definitions;
mod valuables;

// #[cfg(feature="serde")]
// mod serde_sup;

#[cfg(feature="parser")]
lalrpop_mod!(grammar);

#[cfg(feature="parser")]
use lalrpop_util::lalrpop_mod;

pub use valuables::*;
pub use definitions::*;

#[cfg(feature="parser")]
pub use grammar::fileParser as SwearParser;

#[cfg_attr(feature="serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone)]
pub enum TopLevelItem {
	Valuable(Valuable),
	Definition(Definition),
	Dropper(Option<Valuable>),
}

impl From<Valuable> for TopLevelItem {
	fn from(value: Valuable) -> Self {
		TopLevelItem::Valuable(value)
	}
}

impl From<Definition> for TopLevelItem {
	fn from(value: Definition) -> Self {
		TopLevelItem::Definition(value)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	const TEST_SCRIPT: &str = include_str!("../../../testing_script.sw");

	/// Test parsing a basic script.
	#[test]
	fn test_parse() {
		let input = TEST_SCRIPT;
		println!("Testing against input:\n{}", input.split("\n").map(|l| format!("\t{}\n", l)).collect::<String>());

		let parser = SwearParser::new();
		match parser.parse(input) {
			Ok(ast) => println!("Parsed AST:\n{:#?}", ast),
			Err(e) => {
				println!("Error parsing input: {e}");
				panic!();
			}
		}
	}

	/// Test parsing a basic script, then serializing and deserializing it using RON.
	#[test]
	fn test_serde_text() {
		let input = TEST_SCRIPT;
		println!("Testing against input:\n{}", input.split("\n").map(|l| format!("\t{}\n", l)).collect::<String>());

		let parser = SwearParser::new();
		let ast = match parser.parse(input) {
			Ok(ast) => ast,
			Err(e) => {
				println!("Error parsing input: {e}");
				panic!();
			}
		};

		let ron = ron::ser::to_string(&ast).expect("Failed to serialize to RON");
		println!("Serialized to RON:\n{}", ron);

		ron::de::from_str::<Vec<TopLevelItem>>(&ron).expect("Failed to deserialize RON");
		println!("Successfully deserialized RON");
	}

	/// Test parsing a basic script, then serializing and deserializing it using bitcode.
	#[test]
	fn test_serde_bin() {
		let input = TEST_SCRIPT;
		println!("Testing against input:\n{}", input.split("\n").map(|l| format!("\t{}\n", l)).collect::<String>());

		let parser = SwearParser::new();
		let ast = match parser.parse(input) {
			Ok(ast) => ast,
			Err(e) => {
				println!("Error parsing input: {e}");
				panic!();
			}
		};

		let bit = bitcode::serialize(&ast).expect("Failed to serialize to bitcode");
		println!("Serialized to bitcode:\n{:?}", bit);

		bitcode::deserialize::<Vec<TopLevelItem>>(&bit).expect("Failed to deserialize bitcode");
		println!("Successfully deserialized bitcode");
	}
}
