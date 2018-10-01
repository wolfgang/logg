use std::io::prelude::*;
use std::io::BufWriter;
use std::fs::{OpenOptions, File};
use std::path::Path;
use serde_json;

pub (super) fn execute(args: &[String]) {
    init_file_if_needed();

    let mut json: serde_json::Value = ::core::io::get_file_contents_as_json();

    let cat = args[0].clone();
    let body = get_body(args);

    ::core::json::add_entry_to_json(&mut json, &cat, &body);
    write_back_json(&json);
}

fn get_body(args: &[String]) -> String {
    args[1].clone()
}

fn init_file_if_needed() {
    if !Path::new(::core::LOG_FILE).exists() {
		let mut file = File::create(::core::LOG_FILE).expect("Create file failed");
		file.write_all(b"{}").expect("Init file failed");
	}
}

fn write_back_json(json: &serde_json::Value) {
	let file = OpenOptions::new()
					.write(true)
					.truncate(true)
                    .open(::core::LOG_FILE).expect("Open for write failed");
    let mut file = BufWriter::new(file);

    file.write_all(serde_json::to_string_pretty(json).unwrap().as_bytes()).expect("Write failed");

}
