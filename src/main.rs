use swear_lib::runtime::SwearRuntime;

fn main() {
	let time = std::time::Instant::now();
	let parser = swear_lib::swear_parser::SwearParser::new();
	// println!("Time to create parser: {:?}", time.elapsed());
	let time = std::time::Instant::now();
	let script = match parser.parse(include_str!("../testing_script.sw")) {
		Ok(result) => result,
		Err(e) => {
			eprintln!("{e}");
			return;
		}
	};
	// println!("Time to parse: {:?}", time.elapsed());

	// dbg!(&script);
	let mut runtime = swear_lib::runtime::ContextStack::new(script);
	while let Some(_) = runtime.step() {}
}
