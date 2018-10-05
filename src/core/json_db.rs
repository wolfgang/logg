use serde_json;
use chrono::prelude::*;

pub struct JsonDB {
    pub json: serde_json::Value
}

impl JsonDB {
	pub fn new(json: serde_json::Value) -> JsonDB {
		JsonDB { json }
	}

	pub fn set_key(&mut self, key: &str, value: &str) {
		self.json[key] = json!({"value": value});
	}

	pub fn add_entry(&mut self, cat: &str, body: &str) {
		self.add_entry_ext(cat, body, &get_timestamp);
	}

	fn add_entry_ext(
		&mut self,
		cat: &str, 
		body: &str,
		get_timestamp_fn: &Fn() -> i64) 
	{
		if self.json[cat].is_null() {
	    	self.json[cat] = json!({"entries": [new_entry(body, 0, get_timestamp_fn)]});
	    }
	    else {
	    	let entries_ref = &mut self.json[cat]["entries"].as_array_mut().unwrap();
	    	let id = entries_ref.len();
	    	entries_ref.push(new_entry(body, id, get_timestamp_fn));
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

	fn _add_entry(db: &mut JsonDB, cat: &str, body: &str) {
		db.add_entry_ext(cat, body, &get_timestamp_stub);
	}

	fn _body(text: &str, id: usize) -> serde_json::Value {
		json!({"body": text, "id": id, "created_ts": CONST_TIMESTAMP})
	}

    #[test]
    fn can_add_entry_to_empty_json() {
    	let mut db = JsonDB::new(json!({}));
    	_add_entry(&mut db, "some_category", "some_body");
    	assert_eq!(
    		json!({"some_category": {"entries": [_body("some_body", 0)]}}),
    		db.json);
    }

    #[test]
	fn can_add_second_category() {
		let mut db =JsonDB::new(json!({"category_1": {"entries": [_body("body_1", 0)]}}));
		_add_entry(&mut db, "category_2", "body_2");
		assert_eq!(
			json!({
				"category_1": {"entries": [_body("body_1", 0)]},
				"category_2": {"entries": [_body("body_2", 0)]}
			}),
			db.json);
	}
	#[test]
	fn can_add_entry_to_existing_category() {
		let mut db =JsonDB::new(json!({"category_1": {"entries": [_body("body_1", 0)]}}));
		_add_entry(&mut db, "category_1", "body_2");
		assert_eq!(
			json!({"category_1": {"entries": [_body("body_1", 0), _body("body_2", 1)]}}),
			db.json);
	}
}