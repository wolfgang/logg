
use std::io::prelude::*;
use std::io::BufWriter;
use std::fs::OpenOptions;
use std::env;

#[macro_use]
extern crate serde_json;

fn main() -> std::io::Result<()> {
	let args: Vec<String> = env::args().collect();
	println!("{:?}", args);
	let file = OpenOptions::new().append(true).open("logg.txt")?;
    let mut file = BufWriter::new(file);

    let body = format!("{}\n", args[1]);

    let entry_json = json!({
    	"cat": "some category",
    	"body": body
    });


    file.write_all(entry_json.to_string().as_bytes())?;

    Ok(())

}
