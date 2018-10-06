use std::error::Error;
use serde_json;
use ::core::{io, json_filter, display};


pub fn execute(args: &[String]) -> Result<(), Box<Error>> {
	let search_str = get_search_string(args);
	let json: serde_json::Value = io::get_file_contents_as_json();
	let results = json_filter::by_body(search_str, &json);

	if results.len()==1 && results[0].is_unqiue() {
		display::show_entry_for_search_result(&results[0], 0);
		return Ok(())
	}

	for result in results {
		display::show_toc_for_search_result(&result);
	}

	Ok(())
}

fn get_search_string(args: &[String]) -> &str {
	if args.len() == 3 {
		&args[2]
	}
	else {
		""
	}
}
