use core::{io, json_db, json_filter::SearchResult, json_filter::Filter};
use core::error::*;


pub fn create_db_from_log<'a>() -> json_db::JsonDB<'a> {
    let json: serde_json::Value = io::read_log();
    json_db::JsonDB::new(json)
}

pub fn get_requested_category(args: &[String], db: &json_db::JsonDB) -> BoxedResult<SearchResult> {
    let cat = &args[2];
    let result = db.filter_by_category(cat);

    if !result.has_entries() {
        return simple_error(format!("No entries found for category '{}'", cat));
    }
    Ok(result)
}

pub fn parse_requested_id(args: &[String]) -> BoxedResult<usize> {
    let id_str = &args[3];
    id_str.parse::<usize>().or_else({|error|
        simple_error(format!("Could not parse id from '{}': {}", id_str, error))
    })
}

pub fn check_is_valid_id(id: usize, result: &SearchResult) -> EmptyBoxedResult {
    if !result.is_valid_id(id) {
        return simple_error(format!("Id {} is invalid for category '{}'", id, result.category))
    }
    Ok(())
}