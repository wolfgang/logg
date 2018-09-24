use std::process::Command;
use std::env;
use std::fs;

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

pub fn editor_test() {
	let file_name = "/tmp/logg_tmp.txt";
	fs::remove_file(file_name);
	let editor = env::var("EDITOR").expect("EDITOR variable is not set");
	let status = Command::new(editor)
 		.arg(file_name)
 		.status()
 		.expect("Failed to execute editor");

 	println!("vi exited with: {}", status);
	let contents = fs::read_to_string(file_name).expect("Failed to read file contents");
 	println!("edited file contents: {}", contents);

}