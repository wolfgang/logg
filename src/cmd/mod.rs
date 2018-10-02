pub mod add;
pub mod search;

pub fn add(args: &[String]) {
	add::execute(args);
}

pub fn search(args: &[String]) {
	search::execute(args)
}
