use std::error::Error;

pub type BoxedResult<T> = Result<T, Box<Error>>;
pub type EmptyBoxedResult = BoxedResult<()>;

pub fn simple_error(text: String) -> Result<(), Box<Error>> {
	Err(Box::from(text))
}


pub fn simple_error_2(text: String) -> BoxedResult<String> {
	Err(Box::from(text))
}
