use core::error;
use core::error::EmptyBoxedResult;

pub mod add;
pub mod search;
pub mod show;

type CmdFn=&'static Fn(&[String]) -> EmptyBoxedResult;


pub fn add(args: &[String]) -> bool {
	run_cmd(&add::execute, args)
}

pub fn search(args: &[String]) -> bool {
	run_cmd(&search::execute, args)
}

pub fn show(args: &[String]) -> bool {
	run_cmd(&show::execute, args)
}

fn run_cmd(cmd_fn: CmdFn,  args: &[String]) -> bool {
	match cmd_fn(args) {
		Ok(()) => true,
		Err(err) =>  {
			error::print(format!("{}", err));
			false
		}
	}
}
