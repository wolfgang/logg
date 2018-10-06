use std::error::Error;

pub type BoxedResult<T> = Result<T, Box<Error>>;
pub type EmptyBoxedResult = BoxedResult<()>;

pub fn simple_error(text: &str) -> Result<(), Box<Error>> {
	Err(Box::from(text))
}

