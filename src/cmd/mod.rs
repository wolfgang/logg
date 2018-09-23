const LOG_FILE: &'static str = "logg.txt";

pub mod add;

pub fn add(args: &Vec<String>) {
	add::execute(args);
}