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
