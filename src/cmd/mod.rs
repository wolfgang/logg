use core::error::BoxedResult;

pub mod add;
pub mod search;
pub mod show;

type CmdFn=&'static Fn(&[String]) -> BoxedResult;

pub fn run_cmd(cmd_fn: CmdFn,  args: &[String]) -> bool {
	match cmd_fn(args) {
		Ok(()) => true,
		Err(err) =>  {
			println!("Error: {}", err);
			false
		}
	}
}