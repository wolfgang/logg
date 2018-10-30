use core::error::*;
use core::io;
use cmd::utils;

pub (super) fn execute(args: &[String]) -> EmptyBoxedResult {
	let db = utils::create_db_from_log();

	if args.len() < 4 {
		return simple_error("Edit command expected category and id".into());
	}

	let result = utils::get_requested_category(args, &db)?;
	let id = utils::parse_requested_id(args)?;
	utils::check_is_valid_id(id, &result)?;

	let original_body = result.get_body_by_id(id);
	let edited_body = get_edited_body(&original_body)?;
	
	if edited_body == original_body {
		return simple_error("No change was made".into());
	}
	
	let mut db = utils::create_db_from_log();

	db.replace_entry(&result.category, id, &edited_body);
	io::write_log(&db.json);

	Ok(())

}

fn get_edited_body(body: &str) -> BoxedResult<String> {
	::core::editor::get_contents(body)        
}
