use std::fs;
use std::env;
use std::process::Command;
use std::path::Path;
use core::error::*;

const EDITOR_FILE: &'static str = "/tmp/logg_tmp.txt";

pub fn get_contents() -> BoxedResult<String> {
    prepare_editor_file();
    edit_file();
    get_editor_file_contents()
}

fn prepare_editor_file() {
    if Path::new(EDITOR_FILE).exists() {
        fs::remove_file(EDITOR_FILE).expect("Failed to remove temp file");
    }
}

fn edit_file() {
    let editor = env::var("EDITOR").expect("EDITOR variable is not set");
    Command::new(&editor)
        .arg(EDITOR_FILE)
        .status()
        .expect(&format!("Failed to execute editor {}", &editor));
}

fn get_editor_file_contents() -> BoxedResult<String> {
    let contents = match fs::read_to_string(EDITOR_FILE) {
        Ok(contents) => contents,
        Err(err) => return simple_error(format!("Could not get editor contents: {}", err))
    };
    Ok(String::from(contents.trim_right()))
}