use std::error::Error;

pub type BoxedResult=Result<(), Box<Error>>;

pub fn simple_error(text: &str) -> Result<(), Box<Error>> {
	Err(Box::from(text))
}

