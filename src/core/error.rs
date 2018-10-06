use std::error::Error;

pub type BoxedResult<T> = Result<T, Box<Error>>;
pub type EmptyBoxedResult = BoxedResult<()>;

pub fn simple_error<T>(text: String) -> BoxedResult<T> {
	Err(Box::from(text))
}
