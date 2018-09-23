const LOG_FILE: &'static str = "logg.txt";

pub mod add;

pub fn add(args: &[String]) {
	add::execute(args);
}