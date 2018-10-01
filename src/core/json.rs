use serde_json;

pub fn add_entry_to_json(json: &mut serde_json::Value, cat: &str, body: &str) {
	let new_entry: serde_json::Value = json!({"body": body});

    if json[cat].is_null() {
    	json[cat] = json!({"entries": [new_entry]});
    }
    else {
    	let entries_ref = &mut json[cat]["entries"].as_array_mut().unwrap();
    	entries_ref.push(new_entry);
    }
}

pub fn get_body_lines(entry: &serde_json::Value) -> Vec<String> {
	let body = entry["body"].to_string();
	let lines = body.trim_matches('"').split("\\n");
	let mut result:Vec<String> = Vec::new();
	for line in lines {
		result.push(String::from(line));
	}
	result
}

#[cfg(test)]
mod test {
	use super::*;
    #[test]
    fn can_add_entry_to_empty_json() {
    	let mut json = json!({});
    	add_entry_to_json(&mut json, "some_category", "some_body");
    	assert_eq!(
    		json!({"some_category": {"entries": [{"body": "some_body"}]}}),
    		json);
    }
    #[test]
	fn can_add_second_category() {
		let mut json =json!({"category_1": {"entries": [{"body": "body_1"}]}});
		add_entry_to_json(&mut json, "category_2", "body_2");
		assert_eq!(
			json!({
				"category_1": {"entries": [{"body": "body_1"}]},
				"category_2": {"entries": [{"body": "body_2"}]}
			}),
			json);
	}
	#[test]
	fn can_add_entry_to_existing_category() {
		let mut json =json!({"category_1": {"entries": [{"body": "body_1"}]}});
		add_entry_to_json(&mut json, "category_1", "body_2");
		assert_eq!(
			json!({
				"category_1": {
					"entries": [{"body": "body_1"}, {"body": "body_2"}]}
			}),
			json);
	}
	#[test]
	fn get_body_lines_returns_vector_of_lines() {
		let entry = json!({"body": "line1\nline2"});
		assert_eq!(vec!("line1", "line2"), get_body_lines(&entry));
	}
}