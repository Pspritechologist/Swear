#![feature(fn_traits)]

use swear_lib::runtime::SwearRuntime;

fn main() {
	let parser = swear_lib::swear_parser::SwearParser::new();
	let script = match parser.parse(include_str!("../testing_script.sw")) {
		Ok(result) => result,
		Err(e) => {
			eprintln!("{e}");
			return;
		}
	};

	let mut runtime = swear_lib::runtime::ContextStack::new(script);
	while let Some(_) = runtime.step() {}
}
