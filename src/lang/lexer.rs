pub struct Lexer<'a, I: Iterator<Item = char>> {
	src: std::iter::Peekable<&'a mut I>,
	location: super::LocationPoint,
	current: super::LocationPoint,
}

impl<'a, I: Iterator<Item = char>> Lexer<'a, I> {
	pub fn new(src: &'a mut I) -> Self {
		Self {
			src: src.peekable(),
			location: super::LocationPoint::default(),
			current: super::LocationPoint::default(),
		}
	}

	fn eat(&mut self) -> Option<char> {
		let ch = self.src.next()?;

		self.current = self.location;
		self.location.index += 1;

		if ch == '\n' {
			self.location.line += 1;
			self.location.column = 0;
		} else {
			self.location.column += 1;
		}

		Some(ch)
	}
}

impl<'a, I: Iterator<Item = char>> Iterator for Lexer<'a, I> {
	type Item = Result<Token, super::Error>;

	fn next(&mut self) -> Option<Self::Item> {
		while let Some(ch) = self.src.peek() {
			if ch.is_ascii_whitespace() {
				self.eat();
			} else {
				break
			}
		}

		let ch = self.eat()?;
		let start = self.current;

		let kind = match ch {
			'(' => TokenKind::LeftParen,
			')' => TokenKind::RightParen,
			'[' => TokenKind::LeftBracket,
			']' => TokenKind::RightBracket,
			'{' => TokenKind::LeftBrace,
			'}' => TokenKind::RightBrace,
			'.' => TokenKind::Dot,

			'A'..='Z' | 'a'..='z' | '_' => {
				let mut symbol = String::from(ch);
				while let Some('0'..='9' | 'A'..='Z' | 'a'..='z' | '_') = self.src.peek() {
					symbol.push(self.eat().unwrap());
				}

				match symbol.as_str() {
					"func" => TokenKind::Func,
					"def" => TokenKind::Def,

					"true" => TokenKind::Boolean(true),
					"false" => TokenKind::Boolean(false),
					"nil" => TokenKind::Nil,

					_ => TokenKind::Symbol(Symbol::new(symbol).unwrap()),
				}
			},

			'-' | '+' | '0'..='9' => {
				let mut number = String::from(ch);
				while let Some('0'..='9' | '_' | '.' | 'E' | 'e') = self.src.peek() {
					number.push(self.eat().unwrap());
				}

				use std::str::FromStr;
				TokenKind::Number(match f32::from_str(&number) {
					Ok(number) => number,
					Err(_) => return Some(Err(super::Error {
						kind: super::ErrorKind::SyntaxError,
						location: super::LocationArea { start, end: self.current },
						message: "invalid number literal".into(),
					})),
				})
			},
			'"' => {
				todo!()
			},

			ch => return Some(Err(super::Error {
				kind: super::ErrorKind::SyntaxError,
				location: self.current.into(),
				message: format!("unexpected {:?}", ch),
			})),
		};

		if
			kind != TokenKind::LeftParen &&
			kind != TokenKind::LeftBracket &&
			kind != TokenKind::LeftBrace &&
			kind != TokenKind::Dot
		{
			if let Some(ch) = self.src.peek() {
				let exempt = if let TokenKind::Symbol(_) = kind {
					*ch == '.'
				} else {
					false
				};

				if !exempt && !ch.is_ascii_whitespace() {
					return Some(Err(super::Error {
						kind: super::ErrorKind::SyntaxError,
						location: self.location.into(),
						message: "expected delimeter".into(),
					}))
				}
			}
		}

		Some(Ok(Token {
			kind,
			location: super::LocationArea { start, end: self.current },
		}))
	}
}

#[derive(Clone, Debug, PartialEq)]
pub struct Token {
	pub(super) kind: TokenKind,
	pub location: super::LocationArea,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TokenKind {
	LeftParen,
	RightParen,
	LeftBracket,
	RightBracket,
	LeftBrace,
	RightBrace,
	Dot,

	Func,
	Def,

	Symbol(Symbol),

	Number(f32),
	String(String),
	Boolean(bool),
	Nil,
}

#[derive(Clone, Debug, PartialEq, Hash)]
pub struct Symbol(pub(super) String);

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
