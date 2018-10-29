use core::error::*;
use core::io;
use cmd::utils;


pub(super) fn execute(args: &[String]) -> EmptyBoxedResult {
    let mut db = utils::create_db_from_log();

    if args.len() == 0 {
        return simple_error("Please specifiy a category".into())
    }

    let cat = args[0].clone();
    let body = get_body(args)?;

    db.add_entry(&cat, &body);
    io::write_log(&db.json);
    Ok(())
}

fn get_body(args: &[String]) -> BoxedResult<String> {
    if args.len() >=2 {
        Ok(args[1..].join(" "))
    }
    else {
        ::core::editor::get_contents("")        
    }
}
