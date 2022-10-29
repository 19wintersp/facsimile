mod lang;

use lang::{ Value, Symbol };
use lang::runtime::Environment;

use std::fs::File;
use std::io::BufReader;

fn main() {
	let mut args = std::env::args();

	let input_file = match args.nth(1) {
		Some(arg) => arg,
		None => unimplemented!("REPL"),
	};

	let mut input = BufReader::new(match File::open(&input_file) {
		Ok(file) => file,
		Err(error) => {
			eprintln!("Error: {}", error);
			std::process::exit(1);
		},
	});

	let mut prog_args = args
		.map(|arg| Value::String(arg))
		.collect::<Vec<_>>();
	prog_args.insert(0, Value::String(input_file));

	let mut env = Environment {
		symbols: maplit::hashmap! {
			Symbol::new("args".into()).unwrap() => Value::List(prog_args),
		},
		..Default::default()
	};

	if let Err(error) = lang::eval_read(&mut input, Some(&mut env)) {
		eprintln!("{:?}: {}", error.kind, error.message);
		std::process::exit(1);
	}
}
