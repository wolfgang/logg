use serde_json;


pub(super) fn execute(search_str: &str) {
	println!("SEARCH {}", search_str);

	let json: serde_json::Value = ::core::io::get_file_contents_as_json();

}
