use serde_json;
use ::core::{io, json, json_filter, display};

pub(super) fn execute(args: &[String]) {
	let json: serde_json::Value = io::get_file_contents_as_json();
	if args.len()==2 {
		return display::show_toc(&json)
	}

	let cat = &args[2];
	let result = json_filter::by_category(cat, &json);
	if args.len()==4 {
		let index = args[3].parse::<usize>().unwrap();
		let body_as_str =  json::get_body_as_str(result.entries[index]);
		println!("> {}[{}]\n----------\n{}", cat, index, body_as_str);					
	}
	else {
		display::show_toc_for_search_result(&result);
	}
}
