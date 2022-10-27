pub struct Lexer<'a, I: Iterator<Item = char>> {
	src: &'a mut I,
	location: super::LocationPoint,
	current: super::LocationPoint,
}

impl<'a, I: Iterator<Item = char>> Lexer<'a, I> {
	pub fn new(src: &'a mut I) -> Self {
		Self {
			src,
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
				todo!()
			},

			'0'..='9' => {
				todo!()
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

		Some(Ok(Token {
			kind,
			location: super::LocationArea { start, end: self.current },
		}))
	}
}

pub struct Token {
	pub(super) kind: TokenKind,
	pub location: super::LocationArea,
}

pub enum TokenKind {
	LeftParen,
	RightParen,
	LeftBracket,
	RightBracket,
	LeftBrace,
	RightBrace,
	Dot,

	Env,
	Func,
	Let,

	Symbol(Symbol),

	Number(f32),
	String(String),
	Boolean(bool),
	Nil,
}

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
