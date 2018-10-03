use serde_json;
use ::core::{io, json_filter, display};

pub(super) fn execute(args: &[String]) {
	let search_str = get_search_string(args);
	let json: serde_json::Value = io::get_file_contents_as_json();
	let results = json_filter::by_body(search_str, &json);
	for result in results {
		display::show_toc_for_search_result(&result);
	}
}

fn get_search_string(args: &[String]) -> &str {
	if args.len() == 3 {
		&args[2]
	}
	else {
		""
	}
}
