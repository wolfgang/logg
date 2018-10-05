use std::error::Error;

#[test]
fn can_return_error_as_string() {
	assert_eq!(1234, return_error_as_string(false).unwrap());

	let result = match return_error_as_string(true) {
		Ok(_) => String::from("INVALID"),
		Err(error) =>  error.description().to_string()
	};

	assert_eq!("ERROR_STRING", result);

}

fn return_error_as_string(return_error: bool) -> Result<i32, Box<Error>> {
	if return_error {
		return Err("ERROR_STRING".into());
	}
	Ok(1234)

}