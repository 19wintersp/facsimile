use super::{ Value, Error, ErrorKind };
use super::lexer::{ Token, TokenKind };

use std::iter::Peekable;

pub fn parse(
	src: &mut impl Iterator<Item = Token>,
) -> Result<Vec<Value>, Error> {
	parse_impl(&mut src.peekable())
}

fn parse_impl(
	src: &mut Peekable<&mut impl Iterator<Item = Token>>,
) -> Result<Vec<Value>, Error> {
	let mut values = Vec::new();

	loop {
		let next = src.peek();
		if next.is_none() || next.unwrap().kind == TokenKind::RightParen {
			break
		}

		let token = src.next().unwrap();
		values.push(match &token.kind {
			TokenKind::LeftParen => {
				let list = parse_impl(src)?;
				assert_eq!(src.next().unwrap().kind, TokenKind::RightParen);
				Value::List(list)
			},

			TokenKind::RightParen => unreachable!(),

			TokenKind::LeftBracket | TokenKind::RightBracket => return Err(Error {
				kind: ErrorKind::SyntaxError,
				location: token.location,
				message: "unexpected unimplemented bracket".into(),
			}),
			TokenKind::LeftBrace | TokenKind::RightBrace => return Err(Error {
				kind: ErrorKind::SyntaxError,
				location: token.location,
				message: "unexpected unimplemented brace".into(),
			}),
			TokenKind::Dot => return Err(Error {
				kind: ErrorKind::SyntaxError,
				location: token.location,
				message: "unexpected unimplemented path delimeter".into(),
			}),

			TokenKind::Symbol(symbol) => Value::Symbol(symbol.clone()),

			TokenKind::Number(value) => Value::Number(*value),
			TokenKind::String(value) => Value::String(value.clone()),
			TokenKind::Boolean(value) => Value::Boolean(*value),
			TokenKind::Nil => Value::List(Vec::new()),
		});
	}

	Ok(values)
}
