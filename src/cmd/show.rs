use serde_json;
use ::core::{io, json_filter, display};

pub(super) fn execute(args: &[String]) {
	let json: serde_json::Value = io::get_file_contents_as_json();
	if args.len()==2 {
		return display::show_toc(&json)
	}

	let cat = &args[2];
	let result = json_filter::by_category(cat, &json);
	if args.len()==4 {
		let index = args[3].parse::<usize>().unwrap();
		display::show_entry_for_search_result(&result, index);
	}
	else {
		display::show_toc_for_search_result(&result);
	}
}
