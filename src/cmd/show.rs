use serde_json;
use core::{io, json_filter, display};
use core::error::*;


pub(super) fn execute(args: &[String]) -> EmptyBoxedResult {
	let json: serde_json::Value = io::read_log();
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
		let id = parse_id(&args[3])?;
		show_search_result_entry_with_id(&result, id)?
	}
	else {
		display::show_toc_for_search_result(&result);
	}

	Ok(())
}

fn parse_id(id_str: &str) -> BoxedResult<usize> {
	id_str.parse::<usize>().or_else({|error|
		simple_error(format!("Could not parse id from '{}': {}", id_str, error))
	})
}

fn show_search_result_entry_with_id(result: &json_filter::SearchResult, id: usize) -> EmptyBoxedResult {
	if !result.is_valid_id(id) {
		return simple_error(format!("Id {} is invalid for category '{}'", id, result.category))
	}
	display::show_entry_for_search_result(&result, id);
	Ok(())
}