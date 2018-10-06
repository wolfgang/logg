extern crate logg;

use std::env;
use std::process;
use logg::cmd;


fn main() -> std::io::Result<()> {
	let args: Vec<String> = env::args().collect();

	let cmd = &args[1];

	let result = match cmd as &str {
		"a" | "add" => cmd_add(&args[2..]),
		// "f" | "find" => cmd::search::execute(&args),
		// "s" | "show" => cmd::show::execute(&args),
		_ => {
			println!("Invalid command: {}", args[1]);
			false
		}
	};

	if !result {
		process::exit(1);
	}


    Ok(())
}

fn cmd_add(args: &[String]) -> bool {
	match cmd::add::execute(args) {
		Ok(()) => true,
		Err(err) =>  {
			println!("Error: {}", err);
			false
		}
	}
}