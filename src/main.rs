use rgba2hex::{from_alpha_float, from_alpha_uint, AlphaError};
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

	let alpha = match args.next() {
		Some(a) => a,
		None => {
			let hex: String = from_alpha_uint(red, green, blue, None);
			let hex: &str = hex.as_str();
			println!("{hex}");
			process::exit(0);
		}
	};

	match alpha.parse::<u8>() {
		Ok(a) => {
			let hex: String = from_alpha_uint(red, green, blue, Some(a));
			let hex: &str = hex.as_str();
			println!("{hex}");
			process::exit(0);
		}
		Err(_) => {
			let a = alpha.parse::<f32>().unwrap_or_else(|_| {
				eprintln!(
					"invalid alpha in fourth argument: {} or {}",
					AlphaError::InvalidAlphaUintValue,
					AlphaError::InvalidAlphaFloatValue
				);
				process::exit(1);
			});

			match from_alpha_float(red, green, blue, Some(a)) {
				Ok(hex) => {
					let hex: &str = hex.as_str();
					println!("{hex}");
					process::exit(0);
				}
				Err(err) => {
					eprintln!("invalid alpha in fourth argument: {err}");
					process::exit(1);
				}
			}
		}
	}
}
