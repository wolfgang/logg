use serde_json;

pub(super) fn execute() {
	let json: serde_json::Value = ::core::io::get_file_contents_as_json();

	let obj = json.as_object().unwrap();

	for cat_name in obj.keys() {
		let cat = &obj[cat_name];
		println!("{}", cat_name);
		let entries = &cat["entries"].as_array().unwrap();
		for entry in entries.iter() {
			let lines = ::core::json::get_body_lines(entry);
			for line in lines {
				println!("    {}", line);
			}
		}
	}

}