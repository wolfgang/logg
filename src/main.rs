
use std::io::prelude::*;
use std::io::BufWriter;
use std::fs::OpenOptions;
use std::env;

fn main() -> std::io::Result<()> {
	let args: Vec<String> = env::args().collect();
	println!("{:?}", args);
	let file = OpenOptions::new().append(true).open("logg.txt")?;
    let mut file = BufWriter::new(file);

    let s = format!("{}\n", args[1]);

    file.write_all(s.as_bytes())?;

    Ok(())

}
