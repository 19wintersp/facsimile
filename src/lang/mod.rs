pub mod lexer;
pub mod parser;
pub mod runtime;

mod stdlib;

use std::io::Read;

pub fn eval(src: &mut impl Read) -> Result<Value, Error> {
	// this is temporary! fix me!
	let mut buf = String::new();
	src.read_to_string(&mut buf).unwrap();
	eval_str(&buf)
}

pub fn eval_str(src: &str) -> Result<Value, Error> {
	let mut chars = src.chars();
	let lexer = lexer::Lexer::new(&mut chars);

	//todo
	let parsed = parser::parse(&mut lexer.map(|res| res.unwrap())).unwrap();

	println!("{:#?}", parsed);

	todo!()
}

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
	Number(f32),
	String(String),
	Boolean(bool),
	List(Vec<Self>),
	Symbol(Symbol),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Symbol(String);

impl Symbol {
	pub fn new(src: String) -> Option<Self> {
		if src.len() == 0 || src.chars().next().unwrap().is_ascii_digit() {
			return None
		}

		src
			.chars()
			.all(|ch| ch.is_ascii_alphanumeric() || ch == '_')
			.then(|| Self(src))
	}

	pub fn value(&self) -> &str {
		&self.0
	}

	pub fn unwrap(self) -> String {
		self.0
	}
}


#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Error {
	pub kind: ErrorKind,
	pub location: Option<LocationArea>,
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
