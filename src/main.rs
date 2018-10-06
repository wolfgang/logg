extern crate logg;

use std::env;
use std::error::Error;
use std::process;
use logg::cmd::*;


fn main() -> std::io::Result<()> {
	let args: Vec<String> = env::args().collect();

	let cmd = &args[1];

	let result = match cmd as &str {
		"a" | "add" => run_cmd(&add::execute, &args[2..]),
		"f" | "find" => run_cmd(&search::execute, &args),
		"s" | "show" => run_cmd(&show::execute, &args),
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

type BoxedError=Result<(), Box<Error>>;

type CmdFn=&'static Fn(&[String]) -> BoxedError;

fn run_cmd(cmd_fn: CmdFn,  args: &[String]) -> bool {
	match cmd_fn(args) {
		Ok(()) => true,
		Err(err) =>  {
			println!("Error: {}", err);
			false
		}
	}
}