extern crate logg;

use std::env;
use std::process;
use logg::cmd;
use logg::cmd::*;


fn main() -> std::io::Result<()> {
	let args: Vec<String> = env::args().collect();

	let cmd = &args[1];

	let result = match cmd as &str {
		"a" | "add" => cmd::run_cmd(&add::execute, &args[2..]),
		"f" | "find" => cmd::run_cmd(&search::execute, &args),
		"s" | "show" => cmd::run_cmd(&show::execute, &args),
		_ => {
			println!("Error: Invalid command: {}", args[1]);
			false
		}
	};

	if !result {
		process::exit(1);
	}


    Ok(())
}
