mod lang;

fn main() {
	let _ = lang::eval(&mut File::open(&std::env::args().nth(1).unwrap()));
}
