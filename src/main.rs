
use std::io::prelude::*;
use std::io::{BufWriter, BufReader};
use std::fs::{OpenOptions, File};
use std::env;
use std::path::Path;

#[macro_use]
extern crate serde_json;

const LOG_FILE: &'static str = "logg.txt";

fn main() -> std::io::Result<()> {
	let args: Vec<String> = env::args().collect();

    init_file_if_needed();

    let mut json: serde_json::Value = get_file_contents_as_json();

    let cat = &args[1];
    let body = format!("{}\n", args[2]);

    let new_entry: serde_json::Value = json!({"body": body});

    if json[cat].is_null() {
    	json[cat] = json!({"entries": [new_entry]});
    }
    else {
    	let entries_ref = &mut json[cat]["entries"].as_array_mut().unwrap();
    	entries_ref.push(new_entry);

    }

	let file = OpenOptions::new()
					.write(true)
					.truncate(true)
                    .open(LOG_FILE).expect("Open for write failed");
    let mut file = BufWriter::new(file);

    file.write_all(json.to_string().as_bytes()).expect("Write failed");

    Ok(())

}

fn init_file_if_needed() {
    if !Path::new(LOG_FILE).exists() {
		let mut file = File::create(LOG_FILE).expect("Create file failed");
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
                    .open(LOG_FILE).expect("Open for read failed");
    let mut file_for_read = BufReader::new(file_for_read);

 	file_for_read.read_to_string(result).expect("Read from file failed");
}
