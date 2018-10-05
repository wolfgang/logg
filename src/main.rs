extern crate logg;

use std::env;
use std::process;
use logg::cmd;


fn main() -> std::io::Result<()> {
	let args: Vec<String> = env::args().collect();

	let cmd = &args[1];

	match cmd as &str {
		"a" | "add" => cmd::add(&args[2..]),
		"f" | "find" => cmd::search(&args),
		"s" | "show" => cmd::show(&args),
		_ => {
			println!("Invalid command: {}", args[1]);
			process::exit(1);
		}
	}

    Ok(())
}
