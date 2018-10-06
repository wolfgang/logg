use std::io::prelude::*;
use std::error::Error;
use std::io::BufWriter;
use std::fs::{OpenOptions, File};
use std::path::Path;
use serde_json;

pub (super) fn execute(args: &[String]) -> Result<(), Box<Error>> {
    init_log_if_needed();

    let json: serde_json::Value = ::core::io::get_file_contents_as_json();
    let mut db = ::core::json_db::JsonDB::new(json);

    let cat = args[0].clone();
    let body = get_body(args);

    db.add_entry(&cat, &body);
    write_log(&db.json);
    Ok(())
}

fn get_body(args: &[String]) -> String {
    if args.len() >=2 {
        args[1..].join(" ")
    }
    else {
        ::core::editor::get_contents()
    }
}

fn init_log_if_needed() {
    if !Path::new(::core::LOG_FILE).exists() {
		let mut file = File::create(::core::LOG_FILE).expect("Create file failed");
		file.write_all(b"{}").expect("Init file failed");
	}
}

fn write_log(json: &serde_json::Value) {
	let file = OpenOptions::new()
					.write(true)
					.truncate(true)
                    .open(::core::LOG_FILE).expect("Open for write failed");
    let mut file = BufWriter::new(file);

    file.write_all(serde_json::to_string_pretty(json).unwrap().as_bytes()).expect("Write failed");

}
