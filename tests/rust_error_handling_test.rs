use std::num::ParseIntError;

#[test]
fn handle_parse_error() {
	let x: i64 = parse_i64("1234").unwrap();
	assert_eq!(1234, x);

	let s = match parse_i64("abcd") {
		Err(_e) => String::from("error"),
		Ok(num) => num.to_string()
	};

	assert_eq!("error", s);
}

fn parse_i64(s: &str) -> Result<i64, Box<ParseIntError>> {
	s.parse().or_else(|err: ParseIntError| Err(err.into()))
}