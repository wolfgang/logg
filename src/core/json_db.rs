use serde_json;
use chrono::prelude::*;

pub struct JsonDB<'a> {
    pub json: serde_json::Value,
    get_timestamp_fn: &'a Fn() -> i64

}

impl<'a> JsonDB<'a> {
	pub fn new(json: serde_json::Value) -> JsonDB<'a> {
		JsonDB { json, get_timestamp_fn: &get_timestamp }
	}

	fn _new(json: serde_json::Value, get_timestamp_fn: &'a Fn() -> i64) -> JsonDB<'a> {
		JsonDB { json, get_timestamp_fn }
	}

	pub fn set_key(&mut self, key: &str, value: &str) {
		self.json[key] = json!({"value": value});
	}

	pub fn add_entry(&mut self, cat: &str, body: &str) {
		if self.json[cat].is_null() {
	    	self.json[cat] = json!({"entries": [new_entry(body, 0, self.get_timestamp_fn)]});
	    }
	    else {
	    	let entries_ref = &mut self.json[cat]["entries"].as_array_mut().unwrap();
	    	let id = entries_ref.len();
	    	entries_ref.push(new_entry(body, id, self.get_timestamp_fn));
	    }
	}
}

fn new_entry(body: &str, id: usize, get_timestamp_fn: &Fn() -> i64) -> serde_json::Value {
	json!({"body": body, "id": id, "created_ts": get_timestamp_fn()})
} 

fn get_timestamp() -> i64 {
	let now: DateTime<Local> = Local::now();
	now.timestamp()
}


#[cfg(test)]
mod test {
	use super::*;

	const CONST_TIMESTAMP: i64 = 1234;

	fn get_timestamp_stub() -> i64 {
		CONST_TIMESTAMP
	}

	fn _db<'a>(json: serde_json::Value) -> JsonDB<'a> {
		JsonDB::_new(json, &get_timestamp_stub)
	}

	fn _body(text: &str, id: usize) -> serde_json::Value {
		json!({"body": text, "id": id, "created_ts": CONST_TIMESTAMP})
	}

    #[test]
    fn can_add_entry_to_empty_json() {
    	let mut db = _db(json!({}));
    	db.add_entry("some_category", "some_body");
    	assert_eq!(
    		json!({"some_category": {"entries": [_body("some_body", 0)]}}),
    		db.json);
    }

    #[test]
	fn can_add_second_category() {
		let mut db =_db(json!({"category_1": {"entries": [_body("body_1", 0)]}}));
		db.add_entry("category_2", "body_2");
		assert_eq!(
			json!({
				"category_1": {"entries": [_body("body_1", 0)]},
				"category_2": {"entries": [_body("body_2", 0)]}
			}),
			db.json);
	}
	#[test]
	fn can_add_entry_to_existing_category() {
		let mut db =_db(json!({"category_1": {"entries": [_body("body_1", 0)]}}));
		db.add_entry("category_1", "body_2");
		assert_eq!(
			json!({"category_1": {"entries": [_body("body_1", 0), _body("body_2", 1)]}}),
			db.json);
	}
}