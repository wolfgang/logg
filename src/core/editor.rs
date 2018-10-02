use std::fs;
use std::env;
use std::process::Command;
use std::path::Path;

const EDITOR_FILE: &'static str = "/tmp/logg_tmp.txt";

pub fn get_contents() -> String {
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

    println!("editor exited with: {}", status);
}

fn get_editor_file_contents() -> String {
    let contents = fs::read_to_string(EDITOR_FILE).expect("Failed to read file contents");
    println!("edited file contents: {}", contents);
    String::from(contents.trim_right())
}