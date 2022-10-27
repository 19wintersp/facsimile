pub mod lexer;
pub mod parser;
pub mod runtime;

use std::io::Read;

pub fn eval(src: &mut impl Read) -> Result<runtime::Value, Error> {
	// this is temporary! fix me!
	let mut buf = String::new();
	src.read_to_string(&mut buf).unwrap();
	eval_str(&buf)
}

pub fn eval_str(src: &str) -> Result<runtime::Value, Error> {
	todo!()
}
