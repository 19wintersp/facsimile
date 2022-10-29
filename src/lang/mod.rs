pub mod lexer;
pub mod parser;
pub mod runtime;

mod stdlib;

use std::io::Read;

pub fn eval(
	src: &mut impl Read,
	env: Option<&mut runtime::Environment>,
) -> Result<Value, Error> {
	// this is temporary! fix me!
	let mut buf = String::new();
	src.read_to_string(&mut buf).unwrap();
	eval_str(&buf, env)
}

pub fn eval_str(
	src: &str,
	env: Option<&mut runtime::Environment>,
) -> Result<Value, Error> {
	let mut blank = Default::default();
	let env = env.unwrap_or(&mut blank);

	env.functions.extend(stdlib::index().into_iter());

	let mut chars = src.chars();
	let lexer = lexer::Lexer::new(&mut chars);

	// todo: fix error handling here
	let parsed = parser::parse(&mut lexer.map(|res| res.unwrap())).unwrap();

	let mut last = None;
	for value in parsed {
		last = Some(runtime::run(value, env)?);
	}

	Ok(last.unwrap_or(Value::nil()))
}

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
	Number(f32),
	String(String),
	Boolean(bool),
	List(Vec<Self>),
	Symbol(Symbol),
}

impl Value {
	pub fn nil() -> Self {
		Self::List(Vec::new())
	}

	pub fn type_name(&self) -> &'static str {
		match self {
			Self::Number(_) => "number",
			Self::String(_) => "string",
			Self::Boolean(_) => "boolean",
			Self::List(list) => if list.len() > 0 { "list" } else { "nil" },
			Self::Symbol(_) => "symbol",
		}
	}

	pub fn truthy(&self) -> bool {
		match self {
			Self::Number(number) => *number != 0f32,
			Self::String(string) => string.len() > 0,
			Self::Boolean(boolean) => *boolean,
			Self::List(list) => list.len() > 0,
			Self::Symbol(_) => true,
		}
	}
}

impl Default for Value {
	fn default() -> Self {
		Self::nil()
	}
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

	#[allow(unused)]
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
	NameError,
	ArgumentError,
	TypeError,
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
