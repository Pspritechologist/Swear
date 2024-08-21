mod definitions;
mod valuables;

lalrpop_mod!(grammar);

use lalrpop_util::lalrpop_mod;

pub use valuables::*;
pub use definitions::*;
pub use grammar::fileParser as SwearParser;

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

	#[test]
	fn test_fizzbuzz() {
		let input = include_str!("../../../testing_script.sw");
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
}
