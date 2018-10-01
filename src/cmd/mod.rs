pub mod add;
pub mod list;
pub mod search;

pub fn add(args: &[String]) {
	add::execute(args);
}

pub fn list() {
	list::execute();
}

pub fn search(search_str: &str) {
	search::execute(search_str)
}
