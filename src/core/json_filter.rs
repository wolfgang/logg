use serde_json;
use std::collections::HashMap;

pub fn by_category(cat: &str, json: &serde_json::Value) -> serde_json::Value {
	let mut result = json!({});

	let obj = json.as_object().unwrap();

	for k in obj.keys() {
		if cat == "" || cat == k {
			let s = obj[k].to_string();
			result[k] = serde_json::from_str(&s).unwrap();
		}
	}

	let json_str =  result.to_string();
	serde_json::from_str(&json_str).unwrap()
}

pub fn by_body<'a>(search_str: &str, json: &'a serde_json::Value) -> HashMap<&'a String, &'a serde_json::Value> {
	let mut result = HashMap::new();

	let obj = json.as_object().unwrap();

	for cat_name in obj.keys() {
		let cat = &obj[cat_name];
		let entries = &cat["entries"];
		result.insert(cat_name, entries);

	}
	result

}



#[cfg(test)]
mod test {
	use super::*;
    #[test]
    fn return_full_input_if_cat_is_empty_string() {
    	let json = json!({"cat1": {"entries": [{"body": "some body"}]}});
    	let json_result = by_category("", &json);
    	assert_eq!(json, json_result);
    }

    #[test]
    fn return_full_only_give_category() {
    	let json = json!({
    		"cat1": {"entries": [{"body": "some body"}]},
    		"cat2": {"entries": [{"body": "some body cat2"}]},
    	});
		let expected_result = json!({
    		"cat2": {"entries": [{"body": "some body cat2"}]}
    	});

    	let json_result = by_category("cat2", &json);
    	assert_eq!(expected_result, json_result);
    }

}
