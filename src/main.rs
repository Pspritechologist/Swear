fn main() {
	let script = match swear_lib::swear_parser::SwearParser::new().parse(include_str!("../testing_script.sw")) {
		Ok(result) => result,
		Err(e) => {
			eprintln!("{e}");
			return;
		}
	};

	let mut cont = swear_lib::context::RootContext::new();
	let result = swear_lib::execute_expression(script, &mut cont);

	println!("{:?}", result);
}
