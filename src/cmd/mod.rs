pub mod add;
pub mod search;

pub fn add(args: &[String]) {
	add::execute(args);
}

pub fn search(search_str: &str) {
	search::execute(search_str)
}
