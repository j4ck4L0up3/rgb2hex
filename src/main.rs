use std::env;

fn main() {
	let mut args = env::args();

	args.next(); // ignore first arg

	let red = match args.next() {
		Some(val) => val
			.parse::<u8>()
			.unwrap_or_else(|err| panic!("invalid input for red value: {}", err)),
		_ => panic!("expected red value as second argument"),
	};

	let green = match args.next() {
		Some(val) => val
			.parse::<u8>()
			.unwrap_or_else(|err| panic!("invalid input for green value: {}", err)),
		_ => panic!("expected green value as second argument"),
	};

	let blue = match args.next() {
		Some(val) => val
			.parse::<u8>()
			.unwrap_or_else(|err| panic!("invalid input for blue value: {}", err)),
		_ => panic!("expected blue value as second argument"),
	};

	println!("#{:X}{:X}{:X}", red, green, blue);
}
