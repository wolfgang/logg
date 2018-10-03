use serde_json;
use ::core::{io, json, json_filter, display};

pub(super) fn execute(args: &[String]) {
	let json: serde_json::Value = io::get_file_contents_as_json();
	if args.len()==2 {
		return show_all(&json)
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

fn show_all(json: &serde_json::Value) {
	let results = json_filter::by_body("", json);
	for result in results {
		println!("> {} ({})", result.category, result.entries.len());
	}
}
