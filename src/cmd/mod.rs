pub mod add;
pub mod list;

pub fn add(args: &[String]) {
	add::execute(args);
}

pub fn list() {
	list::execute();
}