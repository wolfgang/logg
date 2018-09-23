
extern crate logg;

use std::env;
use logg::cmd;

fn main() -> std::io::Result<()> {
	let args: Vec<String> = env::args().collect();

    cmd::add::execute(&args);

    Ok(())

}
