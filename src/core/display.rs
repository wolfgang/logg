use serde_json;
use ::core::{json, json_filter};

pub fn show_toc(json: &serde_json::Value) {
	let results = json_filter::by_body("", json);
	for result in results {
		println!("> {} ({})", result.category, result.entries.len());
	}
}

pub fn show_toc_for_search_result(result: &json_filter::SearchResult) {
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
