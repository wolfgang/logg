use serde_json;


pub(super) fn execute(args: &[String]) {
	let json: serde_json::Value = ::core::io::get_file_contents_as_json();
	let results = ::core::json_filter::by_body("", &json);
	for result in results {
		println!("> {} ({})", result.category, result.entries.len());
	}
}
