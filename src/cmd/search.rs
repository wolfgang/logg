use serde_json;
use ::core::{io, json, json_filter, display};

pub(super) fn execute(args: &[String]) {
	let search_str = get_search_string(args);
	let json: serde_json::Value = io::get_file_contents_as_json();
	let results = json_filter::by_body(search_str, &json);
	for result in results {
		println!("> {}\n==========", result.category);
		for entry in result.entries {
			println!("{}\n-----", json::get_body_as_str(&entry));
		}
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
