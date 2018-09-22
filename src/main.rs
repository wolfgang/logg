
use std::io::prelude::*;
use std::io::{BufWriter, BufReader};
use std::fs::{OpenOptions, File};
use std::fs;
use std::env;

extern crate serde_json;

fn main() -> std::io::Result<()> {
	let args: Vec<String> = env::args().collect();
 	let contents = fs::read_to_string("logg.txt")?;
	let file = OpenOptions::new()
					.write(true)
					.truncate(true)
                    .open("logg.txt").expect("Open failed");
    let mut file = BufWriter::new(file);

    let body = format!("{}\n", args[1]);

    let mut json: serde_json::Value = serde_json::from_str(&contents).unwrap();
    json["some_category"]["entries"][0]["body"] = serde_json::Value::String(body);

    println!("{:?}", json.to_string());

    file.write_all(json.to_string().as_bytes()).expect("Write failed");

    Ok(())

}
