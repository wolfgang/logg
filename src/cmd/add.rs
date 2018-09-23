use std::io::prelude::*;
use std::io::{BufWriter, BufReader};
use std::fs::{OpenOptions, File};
use std::path::Path;
use serde_json;
use ::add_entry_to_json;

pub (super) fn execute(args: &Vec<String>) {
    init_file_if_needed();

    let mut json: serde_json::Value = get_file_contents_as_json();

    let cat = &args[1];
    let body = &args[2];

    add_entry_to_json(&mut json, cat, body);
    write_back_json(&json);
}

fn init_file_if_needed() {
    if !Path::new(::cmd::LOG_FILE).exists() {
		let mut file = File::create(::cmd::LOG_FILE).expect("Create file failed");
		file.write_all(b"{}").expect("Init file failed");
	}
}

fn get_file_contents_as_json() -> serde_json::Value {
	let mut contents = String::new();
 	get_file_contents(&mut contents);
    serde_json::from_str(&contents).expect("Failed to parse json from file")
}

fn get_file_contents(result: &mut String)  {
	let file_for_read = OpenOptions::new()
					.read(true)
                    .open(::cmd::LOG_FILE).expect("Open for read failed");
    let mut file_for_read = BufReader::new(file_for_read);

 	file_for_read.read_to_string(result).expect("Read from file failed");
}

fn write_back_json(json: &serde_json::Value) {
	let file = OpenOptions::new()
					.write(true)
					.truncate(true)
                    .open(::cmd::LOG_FILE).expect("Open for write failed");
    let mut file = BufWriter::new(file);

    file.write_all(json.to_string().as_bytes()).expect("Write failed");

}