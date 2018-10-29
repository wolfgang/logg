use core::error::*;
use core::error::EmptyBoxedResult;
use core::{io, json_db, json_filter::Filter};


pub (super) fn execute(args: &[String]) -> EmptyBoxedResult {
	let db = get_db();

	let cat = &args[2];
	let result = db.filter_by_category(cat);

	if !result.has_entries() {
		return simple_error(format!("No entries found for category '{}'", cat));
	}

	let id = parse_id(&args[3])?;
	let body_as_str =  result.get_body_by_id(id);


	let new_body = get_edited_body(body_as_str).unwrap();
	
	let mut db = get_db();

	db.replace_entry(cat, id, &new_body);
	io::write_log(&db.json);

	Ok(())

}

fn get_db<'a>() -> json_db::JsonDB<'a> {
	let json: serde_json::Value = io::read_log();
	json_db::JsonDB::new(json)
}

fn parse_id(id_str: &str) -> BoxedResult<usize> {
	id_str.parse::<usize>().or_else({|error|
		simple_error(format!("Could not parse id from '{}': {}", id_str, error))
	})
}

fn get_edited_body(body: &str) -> BoxedResult<String> {
	::core::editor::get_contents(body)        
}
