extern crate logg;
extern crate chrono;

use std::env;
use std::process;
use logg::cmd;
use chrono::prelude::*;


fn main() -> std::io::Result<()> {
	// let utc: DateTime<Local> = Local::now();
	println!("Now: {}", utc);

	let args: Vec<String> = env::args().collect();

	let cmd = &args[1];

	match cmd as &str {
		"a" => cmd::add(&args[2..]),
		"f" => cmd::search(&args),
		"show" => cmd::show(&args),
		_ => {
			println!("Invalid command: {}", args[1]);
			process::exit(1);
		}
	}

    Ok(())
}
