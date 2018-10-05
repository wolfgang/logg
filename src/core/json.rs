use serde_json;
use chrono::prelude::*;

pub fn add_entry_to_json(json: &mut serde_json::Value, cat: &str, body: &str) {
	add_entry_to_json_ext(json, cat, body, &get_timestamp);
}

fn add_entry_to_json_ext(
	json: &mut serde_json::Value, 
	cat: &str, 
	body: &str,
	get_timestamp_fn: &Fn() -> i64) 
{
	if json[cat].is_null() {
    	json[cat] = json!({"entries": [new_entry(body, 0, get_timestamp_fn)]});
    }
    else {
    	let entries_ref = &mut json[cat]["entries"].as_array_mut().unwrap();
    	let id = entries_ref.len();
    	entries_ref.push(new_entry(body, id, get_timestamp_fn));
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

	fn add_entry(json: &mut serde_json::Value, cat: &str, body: &str) {
		add_entry_to_json_ext(json, cat, body, &get_timestamp_stub);
	}

    #[test]
    fn can_add_entry_to_empty_json() {
    	let mut json = json!({});
    	add_entry(&mut json, "some_category", "some_body");
    	assert_eq!(
    		json!({"some_category": {"entries": [{"body": "some_body", "id": 0, "created_ts": CONST_TIMESTAMP}]}}),
    		json);
    }

    #[test]
	fn can_add_second_category() {
		let mut json =json!({"category_1": {"entries": [{"body": "body_1", "id": 0, "created_ts": CONST_TIMESTAMP}]}});
		add_entry(&mut json, "category_2", "body_2");
		assert_eq!(
			json!({
				"category_1": {"entries": [{"body": "body_1", "id": 0, "created_ts": CONST_TIMESTAMP}]},
				"category_2": {"entries": [{"body": "body_2", "id": 0, "created_ts": CONST_TIMESTAMP}]}
			}),
			json);
	}
	#[test]
	fn can_add_entry_to_existing_category() {
		let mut json =json!({"category_1": {"entries": [{"body": "body_1", "id": 0, "created_ts": CONST_TIMESTAMP}]}});
		add_entry(&mut json, "category_1", "body_2");
		assert_eq!(
			json!({
				"category_1": {
					"entries": [
						{"body": "body_1", "id": 0, "created_ts": CONST_TIMESTAMP}, 
						{"body": "body_2", "id": 1, "created_ts": CONST_TIMESTAMP}
					]
				}
			}),
			json);
	}
}