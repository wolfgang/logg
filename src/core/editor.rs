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
    let status = Command::new(editor)
        .arg(EDITOR_FILE)
        .status()
        .expect("Failed to execute editor");

    println!("Editor exited with: {}", status);
}

fn get_editor_file_contents() -> BoxedResult<String> {
    let contents = match fs::read_to_string(EDITOR_FILE) {
        Ok(contents) => contents,
        Err(err) => return simple_error_2(format!("Could not get editor contents: {}", err))
    };
    println!("edited file contents: {}", contents);
    Ok(String::from(contents.trim_right()))
}