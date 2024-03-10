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

	fns![
		not, eq, ne, lt, gt, lte, gte, add, sub, mul, div, rem, get, num, fmt, cat,
		print, input, time, srand, rand,
	]
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

fn lt(args: &[Value]) -> Result<Value, Error> {
	if args.len() != 2 {
		return Err(Error {
			kind: ErrorKind::ArgumentError,
			location: None,
			message: "lt requires two arguments".into(),
		})
	}

	let type_error = Err(Error {
		kind: ErrorKind::TypeError,
		location: None,
		message: "lt only takes numbers".into(),
	});

	Ok(Value::Boolean(
		match args[0] {
			Value::Number(number) => number,
			_ => return type_error,
		} <
		match args[1] {
			Value::Number(number) => number,
			_ => return type_error,
		}
	))
}

fn gt(args: &[Value]) -> Result<Value, Error> {
	if args.len() != 2 {
		return Err(Error {
			kind: ErrorKind::ArgumentError,
			location: None,
			message: "gt requires two arguments".into(),
		})
	}

	let type_error = Err(Error {
		kind: ErrorKind::TypeError,
		location: None,
		message: "gt only takes numbers".into(),
	});

	Ok(Value::Boolean(
		match args[0] {
			Value::Number(number) => number,
			_ => return type_error,
		} >
		match args[1] {
			Value::Number(number) => number,
			_ => return type_error,
		}
	))
}

fn lte(args: &[Value]) -> Result<Value, Error> {
	if args.len() != 2 {
		return Err(Error {
			kind: ErrorKind::ArgumentError,
			location: None,
			message: "lte requires two arguments".into(),
		})
	}

	let type_error = Err(Error {
		kind: ErrorKind::TypeError,
		location: None,
		message: "lte only takes numbers".into(),
	});

	Ok(Value::Boolean(
		match args[0] {
			Value::Number(number) => number,
			_ => return type_error,
		} <=
		match args[1] {
			Value::Number(number) => number,
			_ => return type_error,
		}
	))
}

fn gte(args: &[Value]) -> Result<Value, Error> {
	if args.len() != 2 {
		return Err(Error {
			kind: ErrorKind::ArgumentError,
			location: None,
			message: "gte requires two arguments".into(),
		})
	}

	let type_error = Err(Error {
		kind: ErrorKind::TypeError,
		location: None,
		message: "gte only takes numbers".into(),
	});

	Ok(Value::Boolean(
		match args[0] {
			Value::Number(number) => number,
			_ => return type_error,
		} >=
		match args[1] {
			Value::Number(number) => number,
			_ => return type_error,
		}
	))
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

fn rem(args: &[Value]) -> Result<Value, Error> {
	if args.len() < 2 {
		return Err(Error {
			kind: ErrorKind::ArgumentError,
			location: None,
			message: "rem requires at least two arguments".into(),
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
			message: "rem only takes numbers".into(),
		})
	}

	Ok(Value::Number(
		match args[0] {
			Value::Number(number) => number,
			_ => unreachable!(),
		} %
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
	if !(2..=4).contains(&args.len()) {
		return Err(Error {
			kind: ErrorKind::ArgumentError,
			location: None,
			message: "get requires 2-4 arguments".into(),
		})
	}

	let (list, is_string) = match &args[0] {
		Value::List(list) => (list.clone(), false),
		Value::String(string) => (
			string.chars()
				.map(|ch| Value::String(ch.to_string()))
				.collect::<Vec<_>>(),
			true,
		),
		_ => return Err(Error {
			kind: ErrorKind::TypeError,
			location: None,
			message: "get expects a list or string".into(),
		}),
	};

	let indices = args[1..].iter()
		.map(|arg| match arg {
			Value::Number(number) => Ok(number.round() as isize),
			_ => Err(Error {
				kind: ErrorKind::TypeError,
				location: None,
				message: "get expects numerical indices".into(),
			}),
		})
		.collect::<Result<Vec<_>, _>>()?;

	let start = indices[0];
	let end = indices.get(1).to_owned();
	let step = *indices.get(2).unwrap_or(&1);

	if step == 0 {
		return Err(Error {
			kind: ErrorKind::ArgumentError,
			location: None,
			message: "step cannot be zero".into(),
		})
	}

	let end = match end {
		Some(end) => *end,
		/*
		todo: find better syntax for unbounded indexing
		None => if step > 0 {
			list.len() as isize
		} else {
			-1
		},
		*/
		None => start + step.signum(),
	};

	if end == start {
		return if is_string {
			Ok(Value::String("".into()))
		} else {
			Ok(Value::List(Vec::new()))
		}
	}

	if step.signum() != (end - start).signum() {
		return Err(Error {
			kind: ErrorKind::ArgumentError,
			location: None,
			message: "step does not match indices".into(),
		})
	}

	let mut output = Vec::new();
	let mut i = start;

	while (step > 0 && i < end) || (step < 0 && i > end) {
		if i >= 0 {
			output.push(
				list.get(i as usize)
					.cloned()
					.unwrap_or(Value::nil())
			);
		} else {
			output.push(Value::nil());
		}

		i += step;
	}

	Ok(if is_string {
		Value::String(
			output.into_iter()
				.map(|s| match s {
					Value::String(s) => s,
					_ => unreachable!(),
				})
				.collect::<Vec<_>>()
				.concat()
		)
	} else if output.len() == 1 && args.len() == 2 {
		std::mem::take(&mut output[0])
	} else {
		Value::List(output)
	})
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

fn fmt(args: &[Value]) -> Result<Value, Error> {
	let list = Value::List(args.into());
	let combined = if args.len() == 1 { &args[0] } else { &list };
	Ok(Value::String(combined.to_string()))
}

fn cat(args: &[Value]) -> Result<Value, Error> {
	Ok(Value::String(cat_impl(args)?))
}

fn cat_impl(args: &[Value]) -> Result<String, Error> {
	let mut output = String::new();
	for arg in args {
		match arg {
			Value::Number(number) => output.push_str(&number.to_string()),
			Value::String(string) => output.push_str(&string),
			Value::Boolean(boolean) => output.push_str(&boolean.to_string()),
			Value::List(list) => output.push_str(&cat_impl(&list)?),
			_ => return Err(Error {
				kind: ErrorKind::TypeError,
				location: None,
				message: "cannot concatenate symbol".into(),
			}),
		}
	}

	Ok(output)
}

fn print(args: &[Value]) -> Result<Value, Error> {
	let formatted = args.iter().map(|v| v.to_string()).collect::<Vec<_>>();
	println!("{}", formatted.join(""));

	Ok(Value::nil())
}

fn input(args: &[Value]) -> Result<Value, Error> {
	if args.len() > 0 {
		return Err(Error {
			kind: ErrorKind::ArgumentError,
			location: None,
			message: "input takes no arguments".into(),
		})
	}

	let mut buf = String::new();

	if let Err(err) = std::io::stdin().read_line(&mut buf) {
		return Err(Error {
			kind: ErrorKind::IoError,
			location: None,
			message: err.to_string(),
		})
	}

	if buf.ends_with('\n') { buf.pop(); }
	if buf.ends_with('\r') { buf.pop(); }

	Ok(Value::String(buf))
}

fn time(args: &[Value]) -> Result<Value, Error> {
	use std::time::{ SystemTime, UNIX_EPOCH };

	if args.len() > 0 {
		return Err(Error {
			kind: ErrorKind::ArgumentError,
			location: None,
			message: "time takes no arguments".into(),
		})
	}

	Ok(
		SystemTime::now()
			.duration_since(UNIX_EPOCH)
			.map(|d| Value::Number(d.as_secs_f32()))
			.unwrap_or_default()
	)
}

use std::sync::atomic::{ AtomicU64, Ordering };

static RAND_SEED: AtomicU64 = AtomicU64::new(1);

fn srand(args: &[Value]) -> Result<Value, Error> {
	use std::collections::hash_map::DefaultHasher;
	use std::hash::{ Hash, Hasher };

	if args.len() != 1 {
		return Err(Error {
			kind: ErrorKind::ArgumentError,
			location: None,
			message: "srand requires one argument".into(),
		})
	}

	let mut hasher = DefaultHasher::new();
	args[0].hash(&mut hasher);
	RAND_SEED.store(hasher.finish(), Ordering::SeqCst);

	Ok(Value::nil())
}

fn rand(args: &[Value]) -> Result<Value, Error> {
	if args.len() > 0 {
		return Err(Error {
			kind: ErrorKind::ArgumentError,
			location: None,
			message: "rand takes no arguments".into(),
		})
	}

	let mut new_value = 0u64;

	RAND_SEED.fetch_update(
		Ordering::SeqCst,
		Ordering::SeqCst,
		|seed| {
			// values from glibc posix
			const MULTIPLIER: u64 = 25214903917;
			const INCREMENT: u64 = 11;

			new_value = seed.overflowing_mul(MULTIPLIER).0.overflowing_add(11).0;
			Some(new_value)
		},
	).unwrap();

	Ok(Value::Number((new_value & u32::MAX as u64) as f32))
}
