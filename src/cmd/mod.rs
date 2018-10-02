pub mod add;
pub mod search;
pub mod show;

pub fn add(args: &[String]) {
	add::execute(args);
}

pub fn search(args: &[String]) {
	search::execute(args)
}

pub fn show(args: &[String]) {
	show::execute(args)
}
