pub mod lexer;
pub mod parser;
pub mod runtime;

mod stdlib;

use std::io::{ Bytes, Read };

#[allow(unused)]
pub fn eval(
	src: &str,
	env: Option<&mut runtime::Environment>,
) -> Result<Value, Error> {
	let mut blank = runtime::Environment::default();
	let env = env.unwrap_or(&mut blank);
	env.functions.extend(stdlib::index().into_iter());

	let mut chars = src.chars();
	let lexer = lexer::Lexer::new(&mut chars);

	let mut lexer_error = None;
	let result = parser::parse(
		&mut lexer.map_while(|result| match result {
			Ok(token) => Some(token),
			Err(error) => {
				lexer_error = Some(error);
				None
			},
		}),
	);

	if let Some(error) = lexer_error {
		return Err(error)
	}

	let parsed = result?;

	let mut last = None;
	for value in parsed {
		last = Some(runtime::run(value, env)?);
	}

	Ok(last.unwrap_or(Value::nil()))
}

pub fn eval_read(
	src: &mut impl Read,
	env: Option<&mut runtime::Environment>,
) -> Result<Value, Error> {
	let mut blank = runtime::Environment::default();
	let env = env.unwrap_or(&mut blank);
	env.functions.extend(stdlib::index().into_iter());

	struct Utf8Decoder<R> {
		bytes: Bytes<R>,
		buffer: [u8; 4],
		bailed: bool,
	}

	impl<R: Read> Iterator for Utf8Decoder<R> {
		type Item = Result<char, String>;

		fn next(&mut self) -> Option<Self::Item> {
			if self.bailed {
				return None
			}

			for i in 0..4 {
				match self.bytes.next() {
					Some(Ok(byte)) => self.buffer[i] = byte,
					Some(Err(error)) => {
						self.bailed = true;
						return Some(Err(error.to_string()))
					},
					None => if i == 0 {
						return None
					} else {
						self.bailed = true;
						return Some(Err("unterminated utf-8 sequence".into()))
					},
				}

				if let Ok(ch) = std::str::from_utf8(&self.buffer[..i + 1]) {
					return Some(Ok(ch.chars().next().unwrap()))
				}
			}

			self.bailed = true;
			Some(Err("utf-8 error".into()))
		}
	}

	let decoder = Utf8Decoder {
		bytes: src.bytes(),
		buffer: [0; 4],
		bailed: false,
	};

	let mut decoder_error = None;
	let mut mw = decoder.map_while(|result| match result {
		Ok(ch) => Some(ch),
		Err(error) => {
			decoder_error = Some(error);
			None
		},
	});

	let lexer = lexer::Lexer::new(&mut mw);

	let mut lexer_error = None;
	let result = parser::parse(
		&mut lexer.map_while(|result| match result {
			Ok(token) => Some(token),
			Err(error) => {
				lexer_error = Some(error);
				None
			},
		}),
	);

	if let Some(error) = decoder_error {
		return Err(Error {
			kind: ErrorKind::IoError,
			location: None,
			message: error,
		})
	}

	if let Some(error) = lexer_error {
		return Err(error)
	}

	let parsed = result?;

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
