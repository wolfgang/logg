extern crate logg;

use std::env;
use std::process;
use logg::cmd;


fn main() -> std::io::Result<()> {
	let args: Vec<String> = env::args().collect();

	if args.len()==1 {
		println!("Error: no command specified (add/find/show)");
		process::exit(1);
	}

	let cmd = &args[1];

	let result = match cmd as &str {
		"a" | "add" => cmd::add(&args[2..]),
		"f" | "find" => cmd::search(&args),
		"s" | "show" => cmd::show(&args),
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
