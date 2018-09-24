
extern crate logg;

use std::env;
use std::process;
use logg::cmd;

fn main() -> std::io::Result<()> {
	let args: Vec<String> = env::args().collect();

	let cmd = &args[1];

	match cmd as &str {
		"a" => cmd::add(&args[2..]),
		"l" => cmd::list(),
		"s" => cmd::search(&args[2]),
		_ => {
			println!("Invalid command: {}", args[1]);
			process::exit(1);
		}
	}

    Ok(())
}
