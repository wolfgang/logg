use serde_json;

pub fn add_entry_to_json(json: &mut serde_json::Value, cat: &str, body: &str) {
	if json[cat].is_null() {
    	json[cat] = json!({"entries": [new_entry(body, 0)]});
    }
    else {
    	let entries_ref = &mut json[cat]["entries"].as_array_mut().unwrap();
    	let id = entries_ref.len();
    	entries_ref.push(new_entry(body, id));
    }
}

fn new_entry(body: &str, id: usize) -> serde_json::Value {
	json!({"body": body, "id": id})
} 

pub fn get_body_as_str(entry: &serde_json::Value) -> &str {
	entry["body"].as_str().unwrap()
}

pub fn get_id(entry: &serde_json::Value) -> i64 {
	entry["id"].as_i64().unwrap()
}

#[cfg(test)]
mod test {
	use super::*;
    #[test]
    fn can_add_entry_to_empty_json() {
    	let mut json = json!({});
    	add_entry_to_json(&mut json, "some_category", "some_body");
    	assert_eq!(
    		json!({"some_category": {"entries": [{"body": "some_body", "id": 0}]}}),
    		json);
    }
    #[test]
	fn can_add_second_category() {
		let mut json =json!({"category_1": {"entries": [{"body": "body_1", "id": 0}]}});
		add_entry_to_json(&mut json, "category_2", "body_2");
		assert_eq!(
			json!({
				"category_1": {"entries": [{"body": "body_1", "id": 0}]},
				"category_2": {"entries": [{"body": "body_2", "id": 0}]}
			}),
			json);
	}
	#[test]
	fn can_add_entry_to_existing_category() {
		let mut json =json!({"category_1": {"entries": [{"body": "body_1", "id": 0}]}});
		add_entry_to_json(&mut json, "category_1", "body_2");
		assert_eq!(
			json!({
				"category_1": {
					"entries": [{"body": "body_1", "id": 0}, {"body": "body_2", "id": 1}]}
			}),
			json);
	}
	#[test]
	fn get_body_as_str_returns_string_in_entry_body() {
		let entry = json!({"body": "line1\nline2"});
		assert_eq!("line1\nline2", get_body_as_str(&entry));
	}

	#[test]
	fn get_id_returns_id_as_i64() {
		let entry = json!({"id": 1234});
		assert_eq!(1234, get_id(&entry));
	}

}