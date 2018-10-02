use serde_json;


pub(super) fn execute(search_str: &str) {
	let json: serde_json::Value = ::core::io::get_file_contents_as_json();
	let results = ::core::json_filter::by_body(search_str, &json);
	for result in results {
		println!("{}\n---", result.category);
		for entry in result.entries {
			println!("{}", ::core::json::get_body_as_str(&entry));
		}


	}

}
