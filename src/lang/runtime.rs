use super::{ Value, Symbol, Error, ErrorKind };

use std::collections::HashMap;
use std::sync::atomic::{ AtomicUsize, Ordering };

static LAMBDA_COUNTER: AtomicUsize = AtomicUsize::new(0);

pub fn run(value: Value, env: &mut Environment) -> Result<Value, Error> {
	match value {
		Value::List(items) => match &items[0] {
			Value::Symbol(symbol @ Symbol(name)) => match name.as_str() {
				"quote" => Ok(Value::List(items[1..].to_vec())),
				"fun" | "def" => {
					let basis = if name.as_str() == "fun" { 0 } else { 1 };

					let args = match &items[basis + 1] {
						Value::List(list) => list,
						_ => return Err(Error {
							kind: ErrorKind::ArgumentError,
							location: None, // todo
							message: "expected list to define function arguments".into(),
						}),
					};

					if
						args
							.iter()
							.any(|value| match value {
								Value::Symbol(_) => false,
								_ => true,
							})
					{
						return Err(Error {
							kind: ErrorKind::ArgumentError,
							location: None, // todo
							message: "non-symbol found in argument definition".into(),
						})
					}

					let args = args
						.iter()
						.map(|value| match value {
							Value::Symbol(symbol) => symbol.clone(),
							_ => unreachable!(),
						})
						.collect::<Vec<_>>();

					let symbol = if name.as_str() == "fun" {
						Symbol(format!(
							"%{}",
							LAMBDA_COUNTER.fetch_add(1, Ordering::SeqCst),
						))
					} else {
						match &items[1] {
							Value::Symbol(symbol) => symbol.clone(),
							_ => return Err(Error {
								kind: ErrorKind::ArgumentError,
								location: None, // todo
								message: "expected symbol to identify definition".into(),
							}),
						}
					};

					env.functions.insert(
						symbol.clone(),
						Function::Native {
							args,
							body: items[basis + 2..].to_vec(),
						},
					);

					Ok(Value::Symbol(symbol))
				},
				_ => match env.functions.get(&symbol) {
					Some(Function::Native { args, body }) => {
						let (args, body) = (args.clone(), body.clone());

						if items.len() - 1 != args.len() {
							return Err(Error {
								kind: ErrorKind::ArgumentError,
								location: None, // todo
								message: format!(
									"{} arguments provided ({} expected)",
									items.len() - 1, args.len(),
								),
							})
						}

						let mut new_locals = HashMap::new();
						let mut i = 0;
						for arg in args {
							i += 1;
							new_locals.insert(arg.clone(), run(items[i].clone(), env)?);
						}

						let mut new_env = Environment {
							locals: new_locals,
							..env.clone()
						};

						let mut last = None;
						for value in body {
							last = Some(run(value.clone(), &mut new_env)?);
						}

						Ok(last.unwrap())
					},
					Some(Function::Provided(fun)) => (*fun)(
						&items[1..]
							.iter()
							.map(|item| run(item.clone(), env))
							.collect::<Result<Vec<_>, _>>()?
					),
					None => Err(Error {
						kind: ErrorKind::NameError,
						location: None, // todo
						message: format!("no defined function {:?}", &name),
					})
				},
			},
			other => Err(Error {
				kind: ErrorKind::TypeError,
				location: None, // todo
				message: format!("{} is not callable (use quote)", other.type_name())
			}),
		},
		Value::Symbol(symbol) => {
			if let Some(local) = env.locals.get(&symbol) {
				Ok(local.clone())
			} else if let Some(global) = env.symbols.get(&symbol) {
				Ok(global.clone())
			} else {
				Err(Error {
					kind: ErrorKind::NameError,
					location: None, // todo
					message: format!("symbol {:?} not found (use quote)", symbol.value()),
				})
			}
		}
		_ => Ok(value),
	}
}

#[derive(Clone, Default)]
pub struct Environment {
	pub symbols: HashMap<Symbol, Value>,
	pub locals: HashMap<Symbol, Value>,
	pub functions: HashMap<Symbol, Function>,
}

#[derive(Clone)]
pub enum Function {
	Native {
		args: Vec<Symbol>,
		body: Vec<Value>,
	},
	Provided(fn(&[Value]) -> Result<Value, Error>),
}
