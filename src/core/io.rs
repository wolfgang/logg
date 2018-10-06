use std::io::prelude::*;
use std::io::BufReader;
use std::fs::OpenOptions;
use std::env;
use serde_json;
use core;

pub fn get_home_dir() -> String {
	let home = env::var("HOME").unwrap();
	format!("{}/{}", home, core::LOGG_HOME)
}

pub fn get_log_file() -> String {
	format!("{}/{}", get_home_dir(), core::LOG_FILE)
}

pub fn get_file_contents_as_json() -> serde_json::Value {
	let mut contents = String::new();
 	get_file_contents(&mut contents);
    serde_json::from_str(&contents).expect("Failed to parse json from file")
}

fn get_file_contents(result: &mut String)  {
	let file_for_read = OpenOptions::new()
					.read(true)
                    .open(::core::LOG_FILE).expect("Open for read failed");
    let mut file_for_read = BufReader::new(file_for_read);

 	file_for_read.read_to_string(result).expect("Read from file failed");
}
