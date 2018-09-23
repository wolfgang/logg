#[macro_use]
extern crate serde_json;

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
