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
    fs::read_to_string(EDITOR_FILE)
        .or_else({|error| simple_error(format!("Could not get editor contents: {}", error))})
        .and_then({|contents| Ok(String::from(contents.trim_right()))
    })
}