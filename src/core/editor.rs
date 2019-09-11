use std::io::prelude::*;
use std::fs;
use std::fs::File;
use std::env;
use std::process::Command;
use std::path::Path;
use core::error::*;

pub fn get_contents(contents: &str) -> BoxedResult<String> {
    prepare_editor_file(contents);
    edit_file();
    get_editor_file_contents()
}

fn prepare_editor_file(contents: &str) {
    if Path::new(&get_editor_file()).exists() {
        fs::remove_file(&get_editor_file()).expect("Failed to remove temp file");
    }

    if contents != "" {
        let mut file = File::create(&get_editor_file()).expect("Failed to create temp file");
        file.write_all(contents.as_bytes()).expect("Failed to write to editor temp file");
    }
}

fn edit_file() {
    let editor = env::var("EDITOR").expect("EDITOR variable is not set");
    Command::new(&editor)
        .arg(&get_editor_file())
        .status()
        .expect(&format!("Failed to execute editor {}", &editor));
}

fn get_editor_file_contents() -> BoxedResult<String> {
    fs::read_to_string(&get_editor_file())
        .or_else({|error| simple_error(format!("Could not get editor contents: {}", error))})
        .and_then({|contents| Ok(String::from(contents.trim_end()))
    })
}

fn get_editor_file() -> String {
    let temp_dir = match env::var("TMP") {
        Ok(val) => val,
        Err(_) => String::from("/tmp")
    };
    format!("{}/logg_tmp.txt", temp_dir)

}