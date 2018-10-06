use serde_json;
use core::{io, json_filter, display};
use core::error::*;


pub(super) fn execute(args: &[String]) -> EmptyBoxedResult {
	let json: serde_json::Value = io::get_file_contents_as_json();
	if args.len()==2 {
		display::show_toc(&json);
		return Ok(())
	}

	let cat = &args[2];
	let result = json_filter::by_category(cat, &json);

	if !result.has_entries() {
		return simple_error(format!("No entries found for category '{}'", cat));
	}

	if args.len()==4 {
		let id = args[3].parse::<usize>().unwrap();
		if !result.is_valid_id(id) {
			return simple_error(format!("Id {} is invalid for category '{}'", id, cat));
		}
		display::show_entry_for_search_result(&result, id);
	}
	else {
		display::show_toc_for_search_result(&result);
	}

	Ok(())
}
