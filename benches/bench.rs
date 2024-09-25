use swear_lib::runtime::SwearRuntime;

fn main() {
	divan::main();
}

#[divan::bench(args = [6, 12, 24, 32], sample_size = 1, sample_count = 10)]
fn fibonacci(bencher: divan::Bencher, n: i64) {
	let script = format!(
		r"
			fib! n* [
				n>lesseq '1'#* <+
					n<

				a% >fib n#>sub '2'#*<*<
				b% >fib n#>sub '1'#*<*<

				a>add b*<
			]

			>fib '{}'#*<
		",
		n
	);

	let parser = swear_lib::swear_parser::SwearParser::new();
	let ast = parser.parse(&script).unwrap();

	bencher.bench_local(|| {
		let mut runtime = swear_lib::runtime::ContextStack::new(&ast);
		while !runtime.is_finished() {
			runtime.step();
		}

		let result = runtime.get_result().unwrap();
		result.into_count().unwrap().count
	});
}

#[divan::bench(args = [6, 12, 24, 32], sample_size = 1, sample_count = 10)]
fn fibonacci_lua(bencher: divan::Bencher, n: i64) {
	let script = format!(
		r"
			function fib(n)
				if n <= 1 then
					return n
				else
					return fib(n - 2) + fib(n - 1)
				end
			end

			fib({})
		",
		n
	);

	let mut proc = std::process::Command::new("lua");
	proc.arg("-e").arg(&script);

	bencher.bench_local(|| {
		proc.output().unwrap()
	});
}
