use serde_json;
use core::{io, json_filter, display};
use core::error::*;


pub(super) fn execute(args: &[String]) -> EmptyBoxedResult {
	let search_str = get_search_string(args)?;
	let json: serde_json::Value = io::read_log();
	let results = json_filter::by_body(&search_str, &json);

	if results.len()==1 && results[0].is_unqiue() {
		display::show_entry_for_search_result(&results[0], 0);
		return Ok(())
	}

	for result in results {
		display::show_toc_for_search_result(&result);
	}

	Ok(())
}

fn get_search_string(args: &[String]) -> BoxedResult<String> {
	if args.len() >= 3 {
		Ok(args[2..].join(" "))
	}
	else {
		simple_error("Please specify a string to search for".into())
	}
}
