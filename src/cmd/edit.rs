use core::error::*;
use core::io;
use cmd::utils;

pub (super) fn execute(args: &[String]) -> EmptyBoxedResult {
	let db = utils::create_db_from_log();

	let result = utils::get_requested_category(args, &db)?;

	let id = parse_id(&args[3])?;
	let body_as_str =  result.get_body_by_id(id);

	let new_body = get_edited_body(body_as_str)?;
	
	let mut db = utils::create_db_from_log();

	db.replace_entry(&result.category, id, &new_body);
	io::write_log(&db.json);

	Ok(())

}

fn parse_id(id_str: &str) -> BoxedResult<usize> {
	id_str.parse::<usize>().or_else({|error|
		simple_error(format!("Could not parse id from '{}': {}", id_str, error))
	})
}

fn get_edited_body(body: &str) -> BoxedResult<String> {
	::core::editor::get_contents(body)        
}
