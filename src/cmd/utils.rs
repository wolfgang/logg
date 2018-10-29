use core::{io, json_db};

pub fn create_db_from_log<'a>() -> json_db::JsonDB<'a> {
	let json: serde_json::Value = io::read_log();
	json_db::JsonDB::new(json)
}

