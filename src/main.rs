extern crate logg;

use std::io::prelude::*;
use std::env;
use std::process;
use std::fs::{DirBuilder, File};
use std::path::Path;
use logg::cmd;
use logg::core::io;

fn main() -> std::io::Result<()> {
	init_log();

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

fn init_log() {
	if !Path::new(&io::get_home_dir()).exists() {
	DirBuilder::new()
		.recursive(false)
		.create(io::get_home_dir()).unwrap();
	}

	let mut file = File::create(io::get_log_file()).expect("Create file failed");
	file.write_all(b"{}").expect("Init file failed");
}
