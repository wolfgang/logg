use serde_json;
use colored::*;
use ::core::{json_entry, json_filter};
use chrono::prelude::*;


const LINE: &'static str = "-------------------------------------------------------------------------------";


pub fn show_toc(json: &serde_json::Value) {
	let results = json_filter::by_body("", json);
	for result in results {
		println!("> {} {}", result.category, pretty_count(result.entries.len()));
	}
}

pub fn show_toc_for_search_result(result: &json_filter::SearchResult) {
	println!("> {}", result.category);
	for entry in &result.entries {
		let body_as_str =  json_entry::get_body_as_str(&entry);
		let id = json_entry::get_id(&entry);
		let lines: Vec<&str> = body_as_str.lines().collect();
		let more = if lines.len() > 1 { "[...]".yellow() } else { "".yellow() };
		println!("{} {} {}", pretty_id(id), lines[0], more);
	}
}

pub fn show_entry_for_search_result(result: &json_filter::SearchResult, index: usize) {
	let entry = &result.entries[index];
	let body_as_str =  json_entry::get_body_as_str(entry);
	let id = json_entry::get_id(&entry);
	let created_ts = entry["created_ts"].as_i64().unwrap();
	let created_ts_string = Local.timestamp(created_ts, 0).to_string();
	println!("{} {}\nCreated: {}", result.category.dimmed(), pretty_id(id), created_ts_string.yellow());					
	println!("{}\n{}\n{}", LINE, body_as_str, LINE);					
}

fn pretty_count(count: usize) -> ColoredString {
	format!("({})", count).yellow()
}

fn pretty_id<'a>(id: i64) -> ColoredString {
	format!("[{}]", id).yellow()
}