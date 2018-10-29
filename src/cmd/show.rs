use core::{json_filter, display};
use core::error::*;
use cmd::utils;

pub(super) fn execute(args: &[String]) -> EmptyBoxedResult {
	let db = utils::create_db_from_log();

	if args.len()==2 {
		display::show_toc(&db);
		return Ok(())
	}

	let result = utils::get_requested_category(args, &db)?;

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