use core::error::*;
use core::io;
use cmd::utils;

pub (super) fn execute(args: &[String]) -> EmptyBoxedResult {
	let db = utils::create_db_from_log();

	let result = utils::get_requested_category(args, &db)?;
	let id = utils::parse_requested_id(args)?;
	utils::check_is_valid_id(id, &result)?;

	let edited_body = get_edited_body(result.get_body_by_id(id))?;
	
	let mut db = utils::create_db_from_log();

	db.replace_entry(&result.category, id, &edited_body);
	io::write_log(&db.json);

	Ok(())

}

fn get_edited_body(body: &str) -> BoxedResult<String> {
	::core::editor::get_contents(body)        
}
