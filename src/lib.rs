use std::{error::Error, fmt::Display};

#[derive(PartialEq, Eq, Debug)]
pub enum AlphaError {
	InvalidAlphaFloatValue,
	InvalidAlphaUintValue,
}

impl Error for AlphaError {}

impl Display for AlphaError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			AlphaError::InvalidAlphaFloatValue => {
				write!(f, "alpha value must be in range of 0.0 to 1.0 for float")
			}
			AlphaError::InvalidAlphaUintValue => {
				write!(f, "alpha value must be in range of 0 to 255 for uint")
			}
		}
	}
}

pub fn from_alpha_float(
	red: u8,
	green: u8,
	blue: u8,
	alpha: Option<f32>,
) -> Result<String, AlphaError> {
	if let Some(a) = alpha {
		if !(a <= 1.0 && a >= 0.0) {
			return Err(AlphaError::InvalidAlphaFloatValue);
		}

		let a: f32 = a * 255.0;
		// must round up or automatically rounds down
		let a = a.ceil() as u8;

		return Ok(format!("#{:02X}{:02X}{:02X}{:02X}", red, green, blue, a));
	}

	// default alpha, full opacity
	let a = 255;
	return Ok(format!("#{:02X}{:02X}{:02X}{:02X}", red, green, blue, a));
}

pub fn from_alpha_uint(red: u8, green: u8, blue: u8, alpha: Option<u8>) -> String {
	if let Some(a) = alpha {
		return format!("#{:02X}{:02X}{:02X}{:02X}", red, green, blue, a);
	}

	// default alpha, full opacity
	let a = 255;
	return format!("#{:02X}{:02X}{:02X}{:02X}", red, green, blue, a);
}

// tests
#[cfg(test)]
mod tests {
	use super::*;
	struct RGB(u8, u8, u8);

	#[test]
	fn from_alpha_float_with_none() {
		let rgb = RGB(115, 239, 247);
		let alpha = None;

		assert_eq!(
			from_alpha_float(rgb.0, rgb.1, rgb.2, alpha).unwrap(),
			"#73EFF7FF",
		);
	}

	#[test]
	fn from_alpha_float_with_invalid_alpha() {
		let rgb = RGB(115, 239, 247);
		let alpha = Some(8.0);

		let err = match from_alpha_float(rgb.0, rgb.1, rgb.2, alpha) {
			Err(err) => err,
			Ok(c) => panic!("expected error, got: {}", c),
		};

		assert_eq!(err, AlphaError::InvalidAlphaFloatValue);
	}

	#[test]
	fn from_alpha_float_with_valid_alpha() {
		let rgb = RGB(115, 239, 247);
		let alpha = Some(0.5);

		assert_eq!(
			from_alpha_float(rgb.0, rgb.1, rgb.2, alpha).unwrap(),
			"#73EFF780",
		);
	}

	#[test]
	fn from_alpha_uint_with_none() {
		let rgb = RGB(255, 0, 0);
		let alpha = None;

		assert_eq!(from_alpha_uint(rgb.0, rgb.1, rgb.2, alpha), "#FF0000FF");
	}

	#[test]
	fn from_alpha_uint_with_valid_alpha() {
		let rgb = RGB(0, 0, 255);
		let alpha = Some(32);

		assert_eq!(from_alpha_uint(rgb.0, rgb.1, rgb.2, alpha), "#0000FF20")
	}
}
