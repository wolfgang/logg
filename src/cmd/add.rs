use std::io::prelude::*;
use std::io::BufWriter;
use std::fs::OpenOptions;
use serde_json;
use core::error::*;
use core::io;

pub(super) fn execute(args: &[String]) -> EmptyBoxedResult {
    let json: serde_json::Value = ::core::io::read_log();
    let mut db = ::core::json_db::JsonDB::new(json);

    if args.len() == 0 {
        return simple_error("Please specifiy a category".into())
    }

    let cat = args[0].clone();
    let body = get_body(args)?;

    db.add_entry(&cat, &body);
    write_log(&db.json);
    Ok(())
}

fn get_body(args: &[String]) -> BoxedResult<String> {
    if args.len() >=2 {
        Ok(args[1..].join(" "))
    }
    else {
        ::core::editor::get_contents()        
    }
}

fn write_log(json: &serde_json::Value) {
	let file = OpenOptions::new()
					.write(true)
					.truncate(true)
                    .open(io::get_log_file()).expect("Open for write failed");
    let mut file = BufWriter::new(file);

    file.write_all(serde_json::to_string_pretty(json).unwrap().as_bytes()).expect("Write failed");

}
