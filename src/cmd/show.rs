use serde_json;
use ::core::{io, json, json_filter};

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
		show_toc_for_search_result(&result);
	}
}

fn show_all(json: &serde_json::Value) {
	let results = json_filter::by_body("", json);
	for result in results {
		println!("> {} ({})", result.category, result.entries.len());
	}
}

fn show_toc_for_search_result(result: &json_filter::SearchResult) {
	println!("> {}", result.category);
	let mut index = 0;
	for entry in &result.entries {
		let body_as_str =  json::get_body_as_str(&entry);
		let lines: Vec<&str> = body_as_str.lines().collect();
		let more = if lines.len() > 1 { "[...]" } else { "" };
		println!("[{}] {} {}", index, lines[0], more);
		index = index + 1;
	}
}