use super::{ Value, Symbol, Error, ErrorKind };
use super::runtime::Function;

use std::collections::HashMap;

pub fn index() -> HashMap<Symbol, Function> {
	macro_rules! fns {
		[ $( $fn:ident ),* $(,)? ] => {
			maplit::hashmap! {
				$(
					Symbol(stringify!($fn).into()) => Function::Provided($fn),
				)*
			}
		};
	}

	fns![not, eq, ne, add, sub, mul, div, get, num, cat, print, input]
}

fn not(args: &[Value]) -> Result<Value, Error> {
	if args.len() != 1 {
		return Err(Error {
			kind: ErrorKind::ArgumentError,
			location: None,
			message: "not requires one argument".into(),
		})
	}

	Ok(Value::Boolean(!args[0].truthy()))
}

fn eq(args: &[Value]) -> Result<Value, Error> {
	if args.len() < 2 {
		return Err(Error {
			kind: ErrorKind::ArgumentError,
			location: None,
			message: "eq requires at least two arguments".into(),
		})
	}

	let first = &args[0];
	Ok(Value::Boolean(args.iter().all(|item| item == first)))
}

fn ne(args: &[Value]) -> Result<Value, Error> {
	if args.len() < 2 {
		return Err(Error {
			kind: ErrorKind::ArgumentError,
			location: None,
			message: "ne requires at least two arguments".into(),
		})
	}

	for a in 1..args.len() {
		for b in 0..a {
			if args[a] == args[b] {
				return Ok(Value::Boolean(false))
			}
		}
	}

	Ok(Value::Boolean(true))
}

fn add(args: &[Value]) -> Result<Value, Error> {
	if args.len() < 2 {
		return Err(Error {
			kind: ErrorKind::ArgumentError,
			location: None,
			message: "add requires at least two arguments".into(),
		})
	}

	if
		args
			.iter()
			.any(|item| if let Value::Number(_) = item { false } else { true })
	{
		return Err(Error {
			kind: ErrorKind::TypeError,
			location: None,
			message: "add only takes numbers".into(),
		})
	}

	Ok(Value::Number(
		args
			.iter()
			.map(|item| match item {
				Value::Number(number) => number,
				_ => unreachable!(),
			})
			.sum()
	))
}

fn sub(args: &[Value]) -> Result<Value, Error> {
	if args.len() < 2 {
		return Err(Error {
			kind: ErrorKind::ArgumentError,
			location: None,
			message: "sub requires at least two arguments".into(),
		})
	}

	if
		args
			.iter()
			.any(|item| if let Value::Number(_) = item { false } else { true })
	{
		return Err(Error {
			kind: ErrorKind::TypeError,
			location: None,
			message: "sub only takes numbers".into(),
		})
	}

	Ok(Value::Number(
		match args[0] {
			Value::Number(number) => number,
			_ => unreachable!(),
		} -
		args[1..]
			.iter()
			.map(|item| match item {
				Value::Number(number) => number,
				_ => unreachable!(),
			})
			.sum::<f32>()
	))
}

fn mul(args: &[Value]) -> Result<Value, Error> {
	if args.len() < 2 {
		return Err(Error {
			kind: ErrorKind::ArgumentError,
			location: None,
			message: "mul requires at least two arguments".into(),
		})
	}

	if
		args
			.iter()
			.any(|item| if let Value::Number(_) = item { false } else { true })
	{
		return Err(Error {
			kind: ErrorKind::TypeError,
			location: None,
			message: "mul only takes numbers".into(),
		})
	}

	Ok(Value::Number(
		args
			.iter()
			.map(|item| match item {
				Value::Number(number) => number,
				_ => unreachable!(),
			})
			.product()
	))
}

fn div(args: &[Value]) -> Result<Value, Error> {
	if args.len() < 2 {
		return Err(Error {
			kind: ErrorKind::ArgumentError,
			location: None,
			message: "div requires at least two arguments".into(),
		})
	}

	if
		args
			.iter()
			.any(|item| if let Value::Number(_) = item { false } else { true })
	{
		return Err(Error {
			kind: ErrorKind::TypeError,
			location: None,
			message: "div only takes numbers".into(),
		})
	}

	Ok(Value::Number(
		match args[0] {
			Value::Number(number) => number,
			_ => unreachable!(),
		} /
		args[1..]
			.iter()
			.map(|item| match item {
				Value::Number(number) => number,
				_ => unreachable!(),
			})
			.product::<f32>()
	))
}

fn get(args: &[Value]) -> Result<Value, Error> {
	if args.len() != 2 {
		return Err(Error {
			kind: ErrorKind::ArgumentError,
			location: None,
			message: "get requires two arguments".into(),
		})
	}

	let list = match &args[0] {
		Value::List(list) => list,
		_ => return Err(Error {
			kind: ErrorKind::TypeError,
			location: None,
			message: "get expects a list".into(),
		}),
	};

	let index = match args[1] {
		Value::Number(number) => number.round() as usize,
		_ => return Err(Error {
			kind: ErrorKind::TypeError,
			location: None,
			message: "get expects a number index".into(),
		}),
	};

	Ok(list[index].clone())
}

fn num(args: &[Value]) -> Result<Value, Error> {
	if args.len() != 1 {
		return Err(Error {
			kind: ErrorKind::ArgumentError,
			location: None,
			message: "num requires one argument".into(),
		})
	}

	use std::str::FromStr;
	Ok(Value::Number(match &args[0] {
		Value::Number(number) => *number,
		Value::String(string) => match f32::from_str(&string) {
			Ok(number) => number,
			Err(_) => return Ok(Value::nil()),
		},
		Value::Boolean(boolean) => if *boolean { 1f32 } else { 0f32 },
		Value::List(list) => list.len() as f32,
		_ => return Err(Error {
			kind: ErrorKind::TypeError,
			location: None,
			message: "num expects a number, string, boolean, or list".into(),
		}),
	}))
}

fn cat(args: &[Value]) -> Result<Value, Error> {
	Ok(Value::String(cat_impl(args)))
}

fn cat_impl(args: &[Value]) -> String {
	let mut output = String::new();
	for arg in args {
		match arg {
			Value::Number(number) => output.push_str(&number.to_string()),
			Value::String(string) => output.push_str(&string),
			Value::Boolean(boolean) => output.push_str(&boolean.to_string()),
			Value::List(list) => output.push_str(&cat_impl(&list)),
			Value::Symbol(symbol) => output.push_str(symbol.value()),
		}
	}

	output
}

fn print(args: &[Value]) -> Result<Value, Error> {
	println!("{}", cat_impl(args));
	Ok(Value::nil())
}

fn input(_args: &[Value]) -> Result<Value, Error> {
	todo!()
}
