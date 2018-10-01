use std::io::prelude::*;
use std::fs;
use std::env;
use std::process::Command;
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
    if args.len() >=2 {
        args[1].clone()
    }
    else {
        let file_name = "/tmp/logg_tmp.txt";
        fs::remove_file(file_name).expect("Failed to remove temp file");
        let editor = env::var("EDITOR").expect("EDITOR variable is not set");
        let status = Command::new(editor)
            .arg(file_name)
            .status()
            .expect("Failed to execute editor");

        println!("vi exited with: {}", status);
        let contents = fs::read_to_string(file_name).expect("Failed to read file contents");
        println!("edited file contents: {}", contents);
        contents
    }
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
