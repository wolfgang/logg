
use std::io::prelude::*;
use std::io::{BufWriter, BufReader};
use std::fs::OpenOptions;
use std::env;
#[macro_use]
extern crate serde_json;

fn main() -> std::io::Result<()> {
	let args: Vec<String> = env::args().collect();
    let cat = &args[1];
    let body = format!("{}\n", args[2]);

    let mut contents = String::new();
 	get_file_contents(&mut contents)?;


    let mut json: serde_json::Value = serde_json::from_str(&contents).unwrap();

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
                    .open("logg.txt").expect("Open failed");
    let mut file = BufWriter::new(file);

    file.write_all(json.to_string().as_bytes()).expect("Write failed");

    Ok(())

}

fn get_file_contents(result: &mut String) -> Result<(), std::io::Error>  {
	let file_for_read = OpenOptions::new()
					.read(true)
                    .open("logg.txt").expect("Open for read failed");
    let mut file_for_read = BufReader::new(file_for_read);

 	file_for_read.read_to_string(result)?;
 	Ok(())

}
