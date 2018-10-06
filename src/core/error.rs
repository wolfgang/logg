use std::error::Error;
use colored::*;

pub type BoxedResult<T> = Result<T, Box<Error>>;
pub type EmptyBoxedResult = BoxedResult<()>;

pub fn simple_error<T>(text: String) -> BoxedResult<T> {
	Err(Box::from(text))
}

pub fn print(text: String) {
	println!("{}: {}", "Error".red(), text);
}