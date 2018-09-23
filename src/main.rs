
extern crate logg;

use std::env;
use std::process;
use logg::cmd;

fn main() -> std::io::Result<()> {
	let args: Vec<String> = env::args().collect();

	if args[1] == "a" {
    	cmd::add(&args[2..]);
	} else {
		println!("Invalid command: {}", args[1]);
		process::exit(1);
	}

    Ok(())

}
