use std::{env, process};

fn main() {
	let mut args = env::args();

	args.next(); // ignore first arg

	let red = match args.next() {
		Some(val) => val.parse::<u8>().unwrap_or_else(|err| {
			eprintln!("invalid input for red value: {err}");
			process::exit(1);
		}),
		_ => {
			eprintln!("expected red value as first argument");
			process::exit(1);
		}
	};

	let green = match args.next() {
		Some(val) => val.parse::<u8>().unwrap_or_else(|err| {
			eprintln!("invalid input for green value: {err}");
			process::exit(1)
		}),
		_ => {
			eprintln!("expected green value as second argument");
			process::exit(1);
		}
	};

	let blue = match args.next() {
		Some(val) => val.parse::<u8>().unwrap_or_else(|err| {
			eprintln!("invalid input for blue value: {err}");
			process::exit(1);
		}),
		_ => {
			eprintln!("expected blue value as third argument");
			process::exit(1);
		}
	};

	println!("#{:X}{:X}{:X}", red, green, blue);
}
