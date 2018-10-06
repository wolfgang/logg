use std::error::Error;

pub fn simple_error(text: &str) -> Result<(), Box<Error>> {
	Err(Box::from(text))
}

