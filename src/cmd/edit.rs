use core::error::*;
use core::error::EmptyBoxedResult;

pub (super) fn execute(_args: &[String]) -> EmptyBoxedResult {
	simple_error("Edit not yet implemented".into())
}