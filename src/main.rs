extern crate logg;

use std::env;
use std::process;
use logg::cmd;
use logg::core::{io, error};

fn main() -> std::io::Result<()> {
    io::init_log();

    let args: Vec<String> = env::args().collect();

    if args.len()==1 {
        error::print("No command specified (add/edit/find/show)".into());
        process::exit(1);
    }

    let log_file = logg::core::LOG_FILE;

    let cmd = &args[1];

    let result = match cmd as &str {
        "a" | "add" => cmd::add(&args[2..]),
        "f" | "find" => cmd::search(&args),
        "s" | "show" => cmd::show(&args),
        "e" | "edit" => cmd::edit(&args),
        _ => {
            error::print(format!("Error: Invalid command: {}", args[1]));
            false
        }
    };

    if !result {
        process::exit(1);
    }

    Ok(())
}

