use serde_json;
use ::core::{json_entry, json_filter};

pub fn show_toc(json: &serde_json::Value) {
	let results = json_filter::by_body("", json);
	for result in results {
		println!("> {} ({})", result.category, result.entries.len());
	}
}

pub fn show_toc_for_search_result(result: &json_filter::SearchResult) {
	println!("> {}", result.category);
	for entry in &result.entries {
		let body_as_str =  json_entry::get_body_as_str(&entry);
		let id = json_entry::get_id(&entry);
		let lines: Vec<&str> = body_as_str.lines().collect();
		let more = if lines.len() > 1 { "[...]" } else { "" };
		println!("[{}] {} {}", id, lines[0], more);
	}
}
