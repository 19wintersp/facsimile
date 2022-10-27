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

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Error {
	pub kind: ErrorKind,
	pub location: LocationArea,
	pub message: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum ErrorKind {
	SyntaxError,
	IoError,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash)]
pub struct LocationArea {
	pub start: LocationPoint,
	pub end: LocationPoint,
}

impl From<LocationPoint> for LocationArea {
	fn from(point: LocationPoint) -> Self {
		Self { start: point, end: point }
	}
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash)]
pub struct LocationPoint {
	pub index: usize,
	pub line: usize,
	pub column: usize,
}
